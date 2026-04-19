use chrono::{Duration, NaiveDate};
use iced::widget::canvas::{self, Frame, Geometry, Path, Stroke, Text};
use iced::{mouse::Cursor, mouse::Interaction, Point, Rectangle, Size};

use crate::models::HabitWithStats;
use crate::theme::AppTheme;
use crate::Message;

pub struct LineChart<'a> {
    habits: &'a [HabitWithStats],
    selected_habit: Option<String>,
    today: NaiveDate,
    theme: &'a AppTheme,
    days: i64,
    reveal_progress: f32,
}

impl<'a> LineChart<'a> {
    pub fn new(
        habits: &'a [HabitWithStats],
        selected_habit: Option<String>,
        today: NaiveDate,
        theme: &'a AppTheme,
        days: i64,
        reveal_progress: f32,
    ) -> Self {
        Self {
            habits,
            selected_habit,
            today,
            theme,
            days,
            reveal_progress,
        }
    }
}

#[derive(Clone, Copy)]
struct DailyPoint {
    date: NaiveDate,
    completed: u32,
    total: u32,
    completion_rate: f32,
    x: f32,
    y: f32,
}

impl<'a> canvas::Program<Message> for LineChart<'a> {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &iced::Renderer,
        _theme: &iced::Theme,
        bounds: Rectangle,
        cursor: Cursor,
    ) -> Vec<Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());

        if self.habits.is_empty() || self.days < 2 {
            return vec![frame.into_geometry()];
        }

        let start_date = self.today - Duration::days(self.days.saturating_sub(1));

        let habits_to_draw: Vec<&HabitWithStats> = if let Some(selected_id) = &self.selected_habit {
            self.habits
                .iter()
                .filter(|h| h.habit.id == *selected_id)
                .collect()
        } else {
            self.habits.iter().collect()
        };

        if habits_to_draw.is_empty() {
            return vec![frame.into_geometry()];
        }

        let width = bounds.width;
        let height = bounds.height;
        let left = 36.0;
        let right = 14.0;
        let top = 14.0;
        let bottom = 28.0;
        let plot_left = left;
        let plot_right = (width - right).max(plot_left + 1.0);
        let plot_top = top;
        let plot_bottom = (height - bottom).max(plot_top + 1.0);
        let plot_width = (plot_right - plot_left).max(1.0);
        let plot_height = (plot_bottom - plot_top).max(1.0);

        let days = self.days.max(2) as usize;
        let total_segments = (days - 1).max(1) as f32;
        let mut points = Vec::with_capacity(days);

        for day_index in 0..days {
            let date = start_date + Duration::days(day_index as i64);
            let total = habits_to_draw.len() as u32;
            let completed = habits_to_draw
                .iter()
                .filter(|habit| habit_completed_on(habit, date))
                .count() as u32;

            let completion_rate = if total == 0 {
                0.0
            } else {
                completed as f32 / total as f32
            };

            let x = plot_left + (day_index as f32 / total_segments) * plot_width;
            let y = plot_bottom - completion_rate * plot_height;

            points.push(DailyPoint {
                date,
                completed,
                total,
                completion_rate,
                x,
                y,
            });
        }

        let average_rate = points
            .iter()
            .map(|point| point.completion_rate)
            .sum::<f32>()
            / points.len() as f32;

        let line_color = if self.selected_habit.is_some() {
            self.theme.colors.accent_primary
        } else if average_rate >= 0.65 {
            self.theme.colors.success_fg
        } else if average_rate >= 0.4 {
            self.theme.colors.warning_fg
        } else {
            self.theme.colors.error_fg
        };

        let reveal_progress = self.reveal_progress.clamp(0.02, 1.0);
        let reveal_x = plot_left + reveal_progress * plot_width;
        let revealed_points = revealed_polyline_points(&points, reveal_x);

        // Grid lines and y-axis labels.
        for marker in [0.0_f32, 0.25, 0.5, 0.75, 1.0] {
            let y = plot_bottom - marker * plot_height;
            let line = Path::new(|builder| {
                builder.move_to(Point::new(plot_left, y));
                builder.line_to(Point::new(plot_right, y));
            });

            frame.stroke(
                &line,
                Stroke::default()
                    .with_width(if (marker - 0.0).abs() < f32::EPSILON || (marker - 1.0).abs() < f32::EPSILON {
                        1.0
                    } else {
                        0.8
                    })
                    .with_color(self.theme.colors.border_subtle.scale_alpha(0.8)),
            );
        }

        frame.fill_text(Text {
            content: "100%".to_string(),
            position: Point::new(2.0, plot_top + 4.0),
            color: self.theme.colors.text_tertiary,
            size: 11.0.into(),
            ..Text::default()
        });

        frame.fill_text(Text {
            content: "50%".to_string(),
            position: Point::new(8.0, plot_top + plot_height * 0.5 + 4.0),
            color: self.theme.colors.text_tertiary,
            size: 11.0.into(),
            ..Text::default()
        });

        frame.fill_text(Text {
            content: "0%".to_string(),
            position: Point::new(14.0, plot_bottom + 2.0),
            color: self.theme.colors.text_tertiary,
            size: 11.0.into(),
            ..Text::default()
        });

        // Area fill under smoothed trend line.
        let area = Path::new(|builder| {
            if let Some(first) = revealed_points.first().copied() {
                builder.move_to(Point::new(first.x, plot_bottom));
                builder.line_to(first);
                add_smooth_segments(builder, &revealed_points);

                if let Some(last) = revealed_points.last().copied() {
                    builder.line_to(Point::new(last.x, plot_bottom));
                }

                builder.close();
            }
        });

        frame.fill(&area, line_color.scale_alpha(0.16));

        // Smoothed trend line.
        let trend = Path::new(|builder| {
            if let Some(first) = revealed_points.first().copied() {
                builder.move_to(first);
                add_smooth_segments(builder, &revealed_points);
            }
        });

        frame.stroke(
            &trend,
            Stroke::default()
                .with_width(2.6)
                .with_color(line_color),
        );

        for point in &revealed_points {
            frame.fill(&Path::circle(*point, 2.5), line_color);
        }

        // X-axis labels.
        frame.fill_text(Text {
            content: start_date.format("%b %-d").to_string(),
            position: Point::new(plot_left, plot_bottom + 16.0),
            color: self.theme.colors.text_tertiary,
            size: 11.0.into(),
            ..Text::default()
        });

        frame.fill_text(Text {
            content: self.today.format("%b %-d").to_string(),
            position: Point::new(plot_right - 44.0, plot_bottom + 16.0),
            color: self.theme.colors.text_tertiary,
            size: 11.0.into(),
            ..Text::default()
        });

        if let Some(cursor_pos) = cursor.position_in(bounds) {
            let inside_plot = cursor_pos.x >= plot_left
                && cursor_pos.x <= reveal_x.min(plot_right)
                && cursor_pos.y >= plot_top
                && cursor_pos.y <= plot_bottom;

            if inside_plot && !revealed_points.is_empty() {
                let revealed_width = (reveal_x - plot_left).max(1.0);
                let normalized_x = ((cursor_pos.x - plot_left) / revealed_width).clamp(0.0, 1.0);
                let visible_segments = (revealed_points.len().saturating_sub(1)) as f32;
                let hovered_index = (normalized_x * visible_segments).round() as usize;
                let hovered_index = hovered_index.min(revealed_points.len().saturating_sub(1));
                let hovered_point = revealed_points[hovered_index];
                let hovered = points
                    .iter()
                    .copied()
                    .min_by(|a, b| {
                        (a.x - hovered_point.x)
                            .abs()
                            .partial_cmp(&(b.x - hovered_point.x).abs())
                            .unwrap_or(std::cmp::Ordering::Equal)
                    })
                    .unwrap_or(points[0]);

                let guide = Path::new(|builder| {
                    builder.move_to(Point::new(hovered.x, plot_top));
                    builder.line_to(Point::new(hovered.x, plot_bottom));
                });

                frame.stroke(
                    &guide,
                    Stroke::default()
                        .with_width(1.0)
                        .with_color(line_color.scale_alpha(0.35)),
                );

                frame.fill(
                    &Path::circle(Point::new(hovered.x, hovered.y), 5.0),
                    self.theme.colors.surface_base,
                );

                frame.stroke(
                    &Path::circle(Point::new(hovered.x, hovered.y), 5.0),
                    Stroke::default().with_width(2.0).with_color(line_color),
                );

                let headline = format!(
                    "{}  {:.0}%",
                    hovered.date.format("%b %-d"),
                    hovered.completion_rate * 100.0
                );

                let detail = if hovered.total == 1 {
                    if hovered.completed == 1 {
                        "Completed".to_string()
                    } else {
                        "Not completed".to_string()
                    }
                } else {
                    format!("{} of {} habits completed", hovered.completed, hovered.total)
                };

                let max_text_len = headline.len().max(detail.len()) as f32;
                let tooltip_w = (max_text_len * 6.5 + 18.0).clamp(120.0, 260.0);
                let tooltip_h = 46.0;

                let mut tooltip_x = hovered.x + 10.0;
                if tooltip_x + tooltip_w > plot_right {
                    tooltip_x = hovered.x - tooltip_w - 10.0;
                }
                tooltip_x = tooltip_x.max(plot_left);

                let mut tooltip_y = hovered.y - tooltip_h - 10.0;
                if tooltip_y < plot_top {
                    tooltip_y = hovered.y + 10.0;
                }
                if tooltip_y + tooltip_h > plot_bottom {
                    tooltip_y = plot_bottom - tooltip_h;
                }

                let tooltip_bg = Path::rectangle(
                    Point::new(tooltip_x, tooltip_y),
                    Size::new(tooltip_w, tooltip_h),
                );

                frame.fill(&tooltip_bg, self.theme.colors.surface_base);

                frame.stroke(
                    &tooltip_bg,
                    Stroke::default()
                        .with_width(1.0)
                        .with_color(self.theme.colors.border_default),
                );

                frame.fill_text(Text {
                    content: headline,
                    position: Point::new(tooltip_x + 9.0, tooltip_y + 16.0),
                    color: self.theme.colors.text_primary,
                    size: 12.0.into(),
                    ..Text::default()
                });

                frame.fill_text(Text {
                    content: detail,
                    position: Point::new(tooltip_x + 9.0, tooltip_y + 33.0),
                    color: self.theme.colors.text_secondary,
                    size: 11.0.into(),
                    ..Text::default()
                });
            }
        }

        vec![frame.into_geometry()]
    }

    fn mouse_interaction(
        &self,
        _state: &Self::State,
        bounds: Rectangle,
        cursor: Cursor,
    ) -> Interaction {
        if let Some(position) = cursor.position_in(bounds) {
            let reveal_x = 36.0 + (self.reveal_progress.clamp(0.02, 1.0) * (bounds.width - 50.0));
            let in_plot = position.x >= 36.0
                && position.x <= reveal_x.min(bounds.width - 14.0)
                && position.y >= 14.0
                && position.y <= (bounds.height - 28.0);

            if in_plot {
                return Interaction::Crosshair;
            }
        }

        Interaction::default()
    }
}

fn add_smooth_segments(builder: &mut canvas::path::Builder, points: &[Point]) {
    if points.len() < 2 {
        return;
    }

    if points.len() == 2 {
        builder.line_to(points[1]);
        return;
    }

    for i in 0..(points.len() - 1) {
        let p0 = if i == 0 { points[0] } else { points[i - 1] };
        let p1 = points[i];
        let p2 = points[i + 1];
        let p3 = if i + 2 < points.len() {
            points[i + 2]
        } else {
            points[i + 1]
        };

        let control_1 = Point::new(
            p1.x + (p2.x - p0.x) / 6.0,
            p1.y + (p2.y - p0.y) / 6.0,
        );
        let control_2 = Point::new(
            p2.x - (p3.x - p1.x) / 6.0,
            p2.y - (p3.y - p1.y) / 6.0,
        );

        builder.bezier_curve_to(control_1, control_2, p2);
    }
}

fn revealed_polyline_points(points: &[DailyPoint], reveal_x: f32) -> Vec<Point> {
    if points.is_empty() {
        return Vec::new();
    }

    let mut visible = Vec::with_capacity(points.len());

    for (index, point) in points.iter().enumerate() {
        let current = Point::new(point.x, point.y);

        if point.x <= reveal_x {
            visible.push(current);
            continue;
        }

        if index > 0 {
            let previous = points[index - 1];
            if previous.x < reveal_x {
                let delta_x = (point.x - previous.x).max(0.0001);
                let t = ((reveal_x - previous.x) / delta_x).clamp(0.0, 1.0);
                let interpolated = Point::new(
                    previous.x + (point.x - previous.x) * t,
                    previous.y + (point.y - previous.y) * t,
                );
                visible.push(interpolated);
            }
        }

        break;
    }

    if visible.is_empty() {
        visible.push(Point::new(points[0].x, points[0].y));
    }

    visible
}

fn habit_completed_on(habit: &HabitWithStats, date: NaiveDate) -> bool {
    habit
        .history_logs
        .iter()
        .find(|log| log.date == date)
        .map(|log| log.completed)
        .unwrap_or(false)
}

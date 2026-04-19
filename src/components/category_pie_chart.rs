use std::{cmp::Ordering, collections::HashMap};

use iced::alignment::{Horizontal, Vertical};
use iced::widget::canvas::{self, Frame, Geometry, Path, Stroke, Text as CanvasText};
use iced::widget::canvas::path::Arc;
use iced::widget::{column, container, row, text};
use iced::{mouse::{Cursor, Interaction}, Color, Element, Length, Point, Radians, Rectangle};

use crate::lucide::icon_by_name;
use crate::models::{HabitCategory, HabitWithStats};
use crate::theme::AppTheme;
use crate::Message;

pub fn view<'a>(habits: &'a [HabitWithStats], theme: &'a AppTheme) -> Element<'a, Message> {
    let sp = &theme.spacing;
    let ty = &theme.typography;
    let slices = category_slices(habits);

    let chart = iced::widget::canvas::Canvas::new(CategoryPieChart::new(slices.clone(), theme))
        .width(Length::Fixed(420.0))
        .height(Length::Fixed(260.0));

    let legend = column![text("Categories").size(ty.size_heading_md).color(theme.colors.text_primary)]
        .spacing(sp.md);

    let legend = slices.iter().enumerate().fold(legend, |legend, (index, slice)| {
        let color = category_color(index);

        legend.push(
            row![
                container("")
                    .width(12)
                    .height(12)
                    .style(move |_| category_color_block(color)),
                icon_by_name(slice.category.icon_name(), color, 14.0),
                column![
                    text(slice.category.label())
                        .size(ty.size_body_md)
                        .color(theme.colors.text_primary),
                    text(format!("{} habit{}", slice.count, if slice.count == 1 { "" } else { "s" }))
                        .size(ty.size_caption)
                        .color(theme.colors.text_secondary),
                ]
                .spacing(2),
            ]
            .spacing(sp.sm)
            .align_y(iced::Alignment::Center),
        )
    });

    container(
        row![chart, container(legend).width(Length::Fill)]
            .spacing(sp.xl)
            .align_y(iced::Alignment::Center),
    )
    .padding(24)
    .width(Length::Fill)
    .style(|_| crate::theme::section_card(theme))
    .into()
}

#[derive(Clone)]
struct CategorySlice {
    category: HabitCategory,
    count: u32,
}

struct CategoryPieChart<'a> {
    slices: Vec<CategorySlice>,
    theme: &'a AppTheme,
}

impl<'a> CategoryPieChart<'a> {
    fn new(slices: Vec<CategorySlice>, theme: &'a AppTheme) -> Self {
        Self { slices, theme }
    }
}

impl<'a> canvas::Program<Message> for CategoryPieChart<'a> {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &iced::Renderer,
        _theme: &iced::Theme,
        bounds: Rectangle,
        _cursor: iced::mouse::Cursor,
    ) -> Vec<Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());

        if self.slices.is_empty() {
            return vec![frame.into_geometry()];
        }

        let center = Point::new(bounds.width / 2.0, bounds.height / 2.0);
        let radius = (bounds.width.min(bounds.height) * 0.34).max(1.0);
        let total: u32 = self.slices.iter().map(|slice| slice.count).sum();
        let total_f32 = total.max(1) as f32;
        let mut callouts = Vec::with_capacity(self.slices.len());

        let mut start = -std::f32::consts::FRAC_PI_2;

        for (index, slice) in self.slices.iter().enumerate() {
            let sweep = (slice.count as f32 / total_f32) * std::f32::consts::TAU;
            let end = start + sweep;
            let mid = start + (sweep * 0.5);
            let color = category_color(index);
            let anchor = Point::new(
                center.x + mid.cos() * radius,
                center.y + mid.sin() * radius,
            );
            let side = if mid.cos() >= 0.0 {
                CalloutSide::Right
            } else {
                CalloutSide::Left
            };

            let segment = Path::new(|builder| {
                builder.move_to(center);
                builder.arc(Arc {
                    center,
                    radius,
                    start_angle: Radians(start),
                    end_angle: Radians(end),
                });
                builder.close();
            });

            frame.fill(&segment, color);

            callouts.push(Callout {
                label: slice.category.label(),
                color,
                side,
                anchor,
                target_y: center.y + mid.sin() * (radius + 16.0),
                label_y: 0.0,
            });

            start = end;
        }

        frame.stroke(
            &Path::circle(center, radius),
            Stroke::default()
                .with_width(1.0)
                .with_color(self.theme.colors.border_subtle),
        );

            draw_callouts(&mut frame, center, radius, bounds, &callouts, self.theme);

        vec![frame.into_geometry()]
    }

    fn mouse_interaction(
        &self,
        _state: &Self::State,
        _bounds: Rectangle,
        _cursor: Cursor,
    ) -> Interaction {
        Interaction::default()
    }
}

fn category_slices(habits: &[HabitWithStats]) -> Vec<CategorySlice> {
    let mut counts = HashMap::new();

    for habit in habits {
        *counts.entry(habit.habit.category).or_insert(0_u32) += 1;
    }

    HabitCategory::ALL
        .iter()
        .copied()
        .filter_map(|category| {
            let count = counts.get(&category).copied().unwrap_or(0);

            (count > 0).then_some(CategorySlice { category, count })
        })
        .collect()
}

fn category_color(index: usize) -> Color {
    let palette = [
        Color::from_rgb(0.38, 0.59, 0.98),
        Color::from_rgb(0.27, 0.77, 0.58),
        Color::from_rgb(0.97, 0.66, 0.16),
        Color::from_rgb(0.93, 0.39, 0.65),
        Color::from_rgb(0.61, 0.46, 0.94),
        Color::from_rgb(0.98, 0.47, 0.36),
        Color::from_rgb(0.25, 0.72, 0.78),
        Color::from_rgb(0.84, 0.56, 0.15),
        Color::from_rgb(0.49, 0.76, 0.38),
        Color::from_rgb(0.93, 0.31, 0.31),
        Color::from_rgb(0.42, 0.54, 0.95),
    ];

    palette[index % palette.len()]
}

#[derive(Clone, Copy)]
enum CalloutSide {
    Left,
    Right,
}

impl CalloutSide {
    fn sign(self) -> f32 {
        match self {
            Self::Left => -1.0,
            Self::Right => 1.0,
        }
    }

    fn alignment(self) -> Horizontal {
        match self {
            Self::Left => Horizontal::Right,
            Self::Right => Horizontal::Left,
        }
    }
}

#[derive(Clone, Copy)]
struct Callout<'a> {
    label: &'a str,
    color: Color,
    side: CalloutSide,
    anchor: Point,
    target_y: f32,
    label_y: f32,
}

fn draw_callouts(
    frame: &mut Frame,
    center: Point,
    radius: f32,
    bounds: Rectangle,
    callouts: &[Callout<'_>],
    theme: &AppTheme,
) {
    let mut left_callouts: Vec<Callout<'_>> = callouts
        .iter()
        .copied()
        .filter(|callout| matches!(callout.side, CalloutSide::Left))
        .collect();
    let mut right_callouts: Vec<Callout<'_>> = callouts
        .iter()
        .copied()
        .filter(|callout| matches!(callout.side, CalloutSide::Right))
        .collect();

    arrange_callouts(&mut left_callouts, 18.0, bounds.height - 18.0, 22.0);
    arrange_callouts(&mut right_callouts, 18.0, bounds.height - 18.0, 22.0);

    draw_callout_group(frame, center, radius, &left_callouts, theme);
    draw_callout_group(frame, center, radius, &right_callouts, theme);
}

fn arrange_callouts(callouts: &mut [Callout<'_>], min_y: f32, max_y: f32, spacing: f32) {
    if callouts.is_empty() {
        return;
    }

    callouts.sort_by(|a, b| {
        a.target_y
            .partial_cmp(&b.target_y)
            .unwrap_or(Ordering::Equal)
    });

    let mut previous_y = min_y - spacing;

    for callout in callouts.iter_mut() {
        callout.label_y = callout.target_y.max(previous_y + spacing);
        previous_y = callout.label_y;
    }

    let overflow = (callouts.last().unwrap().label_y - max_y).max(0.0);
    if overflow > 0.0 {
        for callout in callouts.iter_mut() {
            callout.label_y -= overflow;
        }
    }

    let underflow = (min_y - callouts.first().unwrap().label_y).max(0.0);
    if underflow > 0.0 {
        for callout in callouts.iter_mut() {
            callout.label_y += underflow;
        }
    }
}

fn draw_callout_group(
    frame: &mut Frame,
    center: Point,
    radius: f32,
    callouts: &[Callout<'_>],
    theme: &AppTheme,
) {
    for callout in callouts {
        let side = callout.side;
        let sign = side.sign();
        let label_x = center.x + sign * (radius + 28.0);
        let elbow_x = center.x + sign * (radius + 12.0);
        let line = Path::new(|builder| {
            builder.move_to(callout.anchor);
            builder.line_to(Point::new(elbow_x, callout.label_y));
            builder.line_to(Point::new(label_x, callout.label_y));
        });

        frame.stroke(
            &line,
            Stroke::default()
                .with_width(1.2)
                .with_color(callout.color.scale_alpha(0.72)),
        );

        frame.fill(&Path::circle(callout.anchor, 2.5), callout.color);

        frame.fill_text(CanvasText {
            content: callout.label.to_string(),
            position: Point::new(label_x, callout.label_y),
            color: theme.colors.text_primary,
            size: 11.0.into(),
            horizontal_alignment: side.alignment(),
            vertical_alignment: Vertical::Center,
            ..CanvasText::default()
        });
    }
}

fn category_color_block(color: Color) -> iced::widget::container::Style {
    iced::widget::container::Style::default()
        .background(color)
        .border(iced::Border {
            radius: 999.0.into(),
            width: 0.0,
            color,
        })
}
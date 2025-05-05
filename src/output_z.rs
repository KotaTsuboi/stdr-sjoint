use crate::input::HJoint;
use crate::output_util::*;
use anyhow::Result;
use dxf::Drawing;

static GAP_BETWEEN_VIEW: f64 = 1000.0;

fn write_base(drawing: &mut Drawing, input: &HJoint) -> Result<()> {
    let b = input.section.b;
    let margin = 1000.0;
    let gap = 10.0;
    let layer = input.layer_name.base.clone();

    write_line(
        drawing,
        -margin,
        GAP_BETWEEN_VIEW,
        -gap / 2.0,
        GAP_BETWEEN_VIEW,
        &layer,
    )?;
    write_line(
        drawing,
        -margin,
        b + GAP_BETWEEN_VIEW,
        -gap / 2.0,
        b + GAP_BETWEEN_VIEW,
        &layer,
    )?;
    write_line(
        drawing,
        margin,
        GAP_BETWEEN_VIEW,
        gap / 2.0,
        GAP_BETWEEN_VIEW,
        &layer,
    )?;
    write_line(
        drawing,
        margin,
        b + GAP_BETWEEN_VIEW,
        gap / 2.0,
        b + GAP_BETWEEN_VIEW,
        &layer,
    )?;

    write_line(
        drawing,
        gap / 2.0,
        GAP_BETWEEN_VIEW,
        gap / 2.0,
        b + GAP_BETWEEN_VIEW,
        &layer,
    )?;
    write_line(
        drawing,
        -gap / 2.0,
        GAP_BETWEEN_VIEW,
        -gap / 2.0,
        b + GAP_BETWEEN_VIEW,
        &layer,
    )?;

    Ok(())
}

fn write_outer_plate(drawing: &mut Drawing, input: &HJoint) -> Result<()> {
    let b = input.section.b;
    let l = input.flange.outer_plate.l;
    let layer = input.layer_name.plate.clone();

    write_line(
        drawing,
        -l / 2.0,
        GAP_BETWEEN_VIEW,
        l / 2.0,
        GAP_BETWEEN_VIEW,
        &layer,
    )?;
    write_line(
        drawing,
        -l / 2.0,
        b + GAP_BETWEEN_VIEW,
        l / 2.0,
        b + GAP_BETWEEN_VIEW,
        &layer,
    )?;
    write_line(
        drawing,
        -l / 2.0,
        GAP_BETWEEN_VIEW,
        -l / 2.0,
        b + GAP_BETWEEN_VIEW,
        &layer,
    )?;
    write_line(
        drawing,
        l / 2.0,
        GAP_BETWEEN_VIEW,
        l / 2.0,
        b + GAP_BETWEEN_VIEW,
        &layer,
    )?;

    Ok(())
}

fn write_flange_bolt(drawing: &mut Drawing, input: &HJoint) -> Result<()> {
    let is_staggered = input.flange.bolt.is_staggered;
    let pc = if is_staggered { 45.0 } else { 60.0 };
    let g1 = input.flange.gauge.g1;
    let g2 = input.flange.gauge.g2;
    let e = 40.0;
    let b = input.section.b;
    let nf = input.flange.bolt.nf;
    let mf = input.flange.bolt.mf;
    let gap = 10.0;
    let ey = if is_staggered {
        (b - g1 - g2 * 2 as f64) / 2.0
    } else {
        (b - g1 - g2 * (mf - 2) as f64) / 2.0
    };
    let x0 = gap / 2.0 + e;
    let y0 = ey;
    let r = input.bolt.diameter as f64 / 2.0;

    for i in 0..nf {
        for j in 0..(mf / 2) {
            let x = x0 + i as f64 * pc;
            let y = if is_staggered {
                y0 + if i % 2 == 1 { g2 } else { 0.0 }
            } else {
                y0 + j as f64 * g2
            };

            let layer = input.layer_name.plate.clone();

            write_circle(drawing, x, y + GAP_BETWEEN_VIEW, r, &layer)?;
            write_circle(drawing, -x, y + GAP_BETWEEN_VIEW, r, &layer)?;
            write_circle(drawing, x, b - y + GAP_BETWEEN_VIEW, r, &layer)?;
            write_circle(drawing, -x, b - y + GAP_BETWEEN_VIEW, r, &layer)?;

            let layer = input.layer_name.bolt.clone();

            write_cross(drawing, x, y + GAP_BETWEEN_VIEW, 20.0, &layer)?;
            write_cross(drawing, -x, y + GAP_BETWEEN_VIEW, 20.0, &layer)?;
            write_cross(drawing, x, b - y + GAP_BETWEEN_VIEW, 20.0, &layer)?;
            write_cross(drawing, -x, b - y + GAP_BETWEEN_VIEW, 20.0, &layer)?;
        }
    }

    Ok(())
}

pub fn write_x_dim(drawing: &mut Drawing, input: &HJoint) -> Result<()> {
    let gap = 10.0;
    let e = 40.0;
    let is_staggered = input.flange.bolt.is_staggered;
    let pc = if is_staggered { 45.0 } else { 60.0 };
    let layer = "0".to_string();
    let text_rotation_angle = 0.0;
    let x0 = gap / 2.0;
    let y0 = GAP_BETWEEN_VIEW;
    write_dimension(
        drawing,
        -gap / 2.0,
        y0,
        gap / 2.0,
        y0,
        text_rotation_angle,
        layer.clone(),
    )?;
    write_dimension(
        drawing,
        gap / 2.0,
        y0,
        gap / 2.0 + e,
        y0,
        text_rotation_angle,
        layer.clone(),
    )?;
    Ok(())
}

pub fn write_z_view(drawing: &mut Drawing, input: &HJoint) -> Result<()> {
    write_base(drawing, input)?;

    write_outer_plate(drawing, input)?;

    write_flange_bolt(drawing, input)?;

    Ok(())
}

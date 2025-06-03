use crate::property_map::PropertyMap;
use crate::svg_element::svg_circle::SVGCircle;
use crate::svg_element::svg_line::SVGLine;
use crate::svg_element::svg_rectangle::SVGRectangle;

pub mod svg_circle;
pub mod svg_line;
pub mod svg_rectangle;

pub struct SVGElement {
    svg: SVG,
    attributes: PropertyMap,
    css: PropertyMap,
}

impl SVGElement {
    pub fn new(svg_type: SVG, attributes: PropertyMap, css: PropertyMap) -> Self {
        Self {
            svg: svg_type,
            attributes,
            css
        }
    }

    pub fn svg(&self) -> &SVG {
        &self.svg
    }

    pub fn attributes(&self) -> &PropertyMap {
        &self.attributes
    }

    pub fn attributes_mut(&mut self) -> &mut PropertyMap {
        &mut self.attributes
    }

    pub fn css(&self) -> &PropertyMap {
        &self.css
    }

    pub fn css_mut(&mut self) -> &mut PropertyMap {
        &mut self.css
    }

    pub fn circle(svg_circle: SVGCircle, attributes: PropertyMap, css: PropertyMap) -> SVGElement {
        Self::new(
            SVG::Circle(svg_circle),
            attributes,
            css
        )
    }

    pub fn rectangle(svg_rectangle: SVGRectangle, attributes: PropertyMap, css: PropertyMap) -> SVGElement {
        Self::new(
            SVG::Rectangle(svg_rectangle),
            attributes,
            css
        )
    }
}

pub enum SVG {
    SVG,
    Group,
    Circle(SVGCircle),
    Ellipse,
    Line(SVGLine),
    Polygon,
    Polyline,
    Rectangle(SVGRectangle),
    Text,
    Image,
    Path,
    ForeignObject,
}

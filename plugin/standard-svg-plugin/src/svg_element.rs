use crate::property_map::PropertyMap;
use crate::svg_element::svg_circle::SVGCircle;
use crate::svg_element::svg_line::SVGLine;
use crate::svg_element::svg_rectangle::SVGRectangle;
use algebra::linear::matrix::Matrix;

pub mod svg_circle;
pub mod svg_line;
pub mod svg_rectangle;

pub struct SVGElement {
    svg: SVG,
    transformations: Option<Matrix<3>>,
    attributes: PropertyMap,
    css: PropertyMap,
}

impl SVGElement {
    pub fn new(svg: SVG, transformations: Option<Matrix<3>>, attributes: PropertyMap, css: PropertyMap) -> Self {
        Self {
            svg,
            transformations,
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

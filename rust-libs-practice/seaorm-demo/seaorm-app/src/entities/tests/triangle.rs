use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "triangle")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub p1: Point,
    pub p2: Point,
    pub p3: Point,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, FromJsonQueryResult)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl Model {
    fn area(&self) -> f64 {
        let a = self.p1.distance_to(&self.p2);
        let b = self.p2.distance_to(&self.p3);
        let c = self.p3.distance_to(&self.p1);
        let s = (a + b + c) / 2.0;
        (s * (s - a) * (s - b) * (s - c)).sqrt()
    }
}

impl Point {
    fn distance_to(&self, p: &Point) -> f64 {
        let dx = self.x - p.x;
        let dy = self.y - p.y;
        (dx * dx + dy * dy).sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_triangle_area() {
        let t = Model {
            id: 1,
            p1: Point { x: 0., y: 0. },
            p2: Point { x: 2., y: 0. },
            p3: Point { x: 0., y: 2. },
        };
        assert!((t.area() - 2.).abs() < 1e-8);
    }
}

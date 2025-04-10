use crate::primitive::{Aabb, Ray};
use crate::triangle::{Triangle, HitRecord};

use glam::Vec3A;

pub const ROOT_IDX: usize = 0;

// Higher amount leads to better BVH at longer construction time.
const SPACES: usize = 10;

#[derive(Debug)]
pub struct Bvh {
    nodes: Vec<BvhNode>,
    pub triangles: Vec<Triangle>,
    pub nodes_used: usize
}

#[derive(Debug, Clone)]
pub struct BvhNode {
    aabb: Aabb,
    left_child: usize,
    first_prim: usize,
    prim_count: usize
}

impl BvhNode {
    pub fn new(first_prim: usize, prim_count: usize, triangles: &Vec<Triangle>) -> Self {
        let mut aabb = Aabb::new(Vec3A::INFINITY, Vec3A::NEG_INFINITY);

        for i in first_prim..first_prim+prim_count {
            let tri = &triangles[i];
            aabb.minimum = aabb.minimum.min(tri.v1.position);
            aabb.minimum = aabb.minimum.min(tri.v2.position);
            aabb.minimum = aabb.minimum.min(tri.v3.position);
            aabb.maximum = aabb.maximum.max(tri.v1.position);
            aabb.maximum = aabb.maximum.max(tri.v2.position);
            aabb.maximum = aabb.maximum.max(tri.v3.position);
        }

        Self {
            aabb,
            left_child: 0,
            first_prim,
            prim_count
        }
    }

    pub fn is_leaf(&self) -> bool {
        self.prim_count > 0
    }

    pub fn evaluate_sh(&self, axis: usize, pos: f32, triangles: &Vec<Triangle>) -> f32 {
        let mut left_box = Aabb::MAX;
        let mut right_box = Aabb::MAX;

        let mut left_count = 0;
        let mut right_count = 0;

        for i in self.first_prim..self.first_prim+self.prim_count {
            let triangle = &triangles[i];
            if triangle.centroid[axis] < pos {
                left_count += 1;
                left_box.grow(triangle.v1.position);
                left_box.grow(triangle.v2.position);
                left_box.grow(triangle.v3.position);
            } else {
                right_count += 1;
                right_box.grow(triangle.v1.position);
                right_box.grow(triangle.v2.position);
                right_box.grow(triangle.v3.position);
            }
        }

        let cost = (left_count as f32) * left_box.area() + (right_count as f32) * right_box.area();
        if cost > 0.0 {
            cost
        } else {
            f32::INFINITY
        }
    }
}

impl Bvh {
    pub fn new(triangles: Vec<Triangle>) -> Self {
        let root = BvhNode::new(0, triangles.len(), &triangles);
        let nodes = vec![root];

        let mut bvh = Bvh {
            nodes,
            triangles,
            nodes_used: 1
        };

        bvh.subdivide(ROOT_IDX);
        bvh
    }

    pub fn intersects(&self, ray: &Ray) -> Option<HitRecord> {
        self.traverse(ray, ROOT_IDX)
    }

    fn traverse(&self, ray: &Ray, node_idx: usize) -> Option<HitRecord> {
        let node = &self.nodes[node_idx];

        if !node.aabb.hit(ray) {
            return None;
        }

        if node.is_leaf() {
            let mut nearest_dist = f32::INFINITY;
            let mut nearest_hit: Option<HitRecord> = None;

            for i in node.first_prim..node.first_prim+node.prim_count {
                let triangle = &self.triangles[i];

                match triangle.hit(&ray) {
                    Some(hit_record) if hit_record.t < nearest_dist => {
                        nearest_dist = hit_record.t;
                        nearest_hit = Some(hit_record);
                    },
                    _ => ()
                }
            }

            return nearest_hit;
        }

        let left_hit = self.traverse(ray, node.left_child);
        let right_hit = self.traverse(ray, node.left_child + 1);

        match (left_hit, right_hit) {
            (Some(t1), Some(t2)) if t1.t <= t2.t => Some(t1),
            (Some(_), Some(t2)) => Some(t2),
            (Some(t1), None) => Some(t1),
            (None, Some(t2)) => Some(t2),
            (None, None) => None
        }
    }

    fn subdivide(&mut self, node_idx: usize) {
        let node = &mut self.nodes[node_idx];

        let mut best_axis = 5;
        let mut best_pos = 0.0;
        let mut best_cost = f32::INFINITY;

        for axis in 0..3 {
            let bounds_min = node.aabb.minimum[axis];
            let bounds_max = node.aabb.maximum[axis];

            if bounds_min == bounds_max {
                continue;
            }

            let scale = (bounds_max - bounds_min) / (SPACES as f32);
            for i in 1..SPACES {
                let candidate_pos = bounds_min + (i as f32) * scale;
                let cost = node.evaluate_sh(axis, candidate_pos, &self.triangles);
                if cost < best_cost {
                    best_pos = candidate_pos;
                    best_axis = axis;
                    best_cost = cost;
                }
            }
        }

        let extent = node.aabb.maximum - node.aabb.minimum;
        let parent_area = extent.x * extent.y + extent.y * extent.z + extent.z * extent.x;
        let parent_cost = (node.prim_count as f32) * parent_area;

        if best_cost >= parent_cost {
            return;
        }

        let axis = best_axis;
        let split_pos = best_pos;

        let mut i = node.first_prim;
        let mut j = i + node.prim_count - 1;

        while i <= j {
            if self.triangles[i].centroid[axis] < split_pos {
                i += 1;
            } else {
                self.triangles.swap(i, j);
                j -= 1;
            }
        }

        let left_count = i - node.first_prim;
        if left_count == 0 || left_count == node.prim_count {
            return;
        }

        let left_child_idx = self.nodes_used;
        let right_child_idx = self.nodes_used + 1;
        self.nodes_used += 2;

        let node_first_prim = node.first_prim;
        let node_prim_count = node.prim_count;

        node.left_child = left_child_idx;
        node.prim_count = 0;

        self.nodes.push(BvhNode::new(node_first_prim, left_count, &self.triangles));
        self.nodes.push(BvhNode::new(i, node_prim_count - left_count, &self.triangles));

        self.subdivide(left_child_idx);
        self.subdivide(right_child_idx);
    }
}

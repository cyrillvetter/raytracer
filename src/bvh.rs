use crate::{
    primitive::{Aabb, Ray},
    triangle::{Triangle, HitRecord}
};

use std::ops::Range;

pub const ROOT_IDX: usize = 0;

// Higher amount leads to better BVH at longer construction time.
const SPACES: usize = 10;

#[derive(Debug)]
pub struct Bvh {
    nodes: Vec<BvhNode>,
    pub triangles: Vec<Triangle>,
    pub nodes_used: usize
}

#[derive(Debug)]
struct BvhNode {
    aabb: Aabb,
    left_child: usize,
    first_tri: usize,
    tri_count: usize
}

impl BvhNode {
    fn new(first_tri: usize, tri_count: usize, triangles: &Vec<Triangle>) -> Self {
        let mut aabb = Aabb::MAX;

        for tri in &triangles[first_tri..first_tri+tri_count] {
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
            first_tri,
            tri_count
        }
    }

    fn is_leaf(&self) -> bool {
        self.tri_count > 0
    }

    fn tri_range(&self) -> Range<usize> {
        self.first_tri..self.first_tri+self.tri_count
    }

    fn evaluate_sah(&self, axis: usize, pos: f32, triangles: &Vec<Triangle>) -> f32 {
        let mut left_box = Aabb::MAX;
        let mut right_box = Aabb::MAX;

        let mut left_count = 0;
        let mut right_count = 0;

        for tri in &triangles[self.first_tri..self.first_tri+self.tri_count] {
            if tri.centroid[axis] < pos {
                left_count += 1;
                left_box.grow(tri.v1.position);
                left_box.grow(tri.v2.position);
                left_box.grow(tri.v3.position);
            } else {
                right_count += 1;
                right_box.grow(tri.v1.position);
                right_box.grow(tri.v2.position);
                right_box.grow(tri.v3.position);
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
        let mut nodes = Vec::with_capacity(triangles.len() * 2 - 1);
        nodes.push(root);

        let mut bvh = Bvh {
            nodes,
            triangles,
            nodes_used: 1
        };

        bvh.subdivide(ROOT_IDX);
        bvh
    }

    pub fn intersects(&self, ray: &Ray) -> Option<HitRecord> {
        let mut node = &self.nodes[ROOT_IDX];
        let mut stack = [node; 64];
        let mut stack_pointer = 0;

        let mut nearest_dist = f32::INFINITY;
        let mut nearest_tri: Option<&Triangle> = None;

        loop {
            if node.is_leaf() {
                for tri in &self.triangles[node.tri_range()] {
                    match tri.hit(ray) {
                        Some(dist) if dist < nearest_dist => {
                            nearest_dist = dist;
                            nearest_tri = Some(tri);
                        },
                        _ => ()
                    }
                }

                if stack_pointer == 0 {
                    break;
                } else {
                    stack_pointer -= 1;
                    node = stack[stack_pointer];
                }

                continue;
            }

            let mut child1 = &self.nodes[node.left_child];
            let mut child2 = &self.nodes[node.left_child + 1];

            let mut dist1 = child1.aabb.hit(ray, nearest_dist);
            let mut dist2 = child2.aabb.hit(ray, nearest_dist);

            if dist1 > dist2 {
                std::mem::swap(&mut dist1, &mut dist2);
                std::mem::swap(&mut child1, &mut child2);
            }

            if dist1 == f32::INFINITY {
                if stack_pointer == 0 {
                    break;
                } else {
                    stack_pointer -= 1;
                    node = stack[stack_pointer];
                }
            } else {
                node = child1;
                if dist2 != f32::INFINITY {
                    stack[stack_pointer] = child2;
                    stack_pointer += 1;
                }
            }
        }

        nearest_tri.map(|tri| tri.create_record(ray, nearest_dist))
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
                let cost = node.evaluate_sah(axis, candidate_pos, &self.triangles);
                if cost < best_cost {
                    best_pos = candidate_pos;
                    best_axis = axis;
                    best_cost = cost;
                }
            }
        }

        let extent = node.aabb.maximum - node.aabb.minimum;
        let parent_area = extent.x * extent.y + extent.y * extent.z + extent.z * extent.x;
        let parent_cost = (node.tri_count as f32) * parent_area;

        if best_cost >= parent_cost {
            return;
        }

        let axis = best_axis;
        let split_pos = best_pos;

        let mut i = node.first_tri;
        let mut j = i + node.tri_count - 1;

        while i <= j {
            if self.triangles[i].centroid[axis] < split_pos {
                i += 1;
            } else {
                self.triangles.swap(i, j);
                j -= 1;
            }
        }

        let left_count = i - node.first_tri;
        if left_count == 0 || left_count == node.tri_count {
            return;
        }

        let left_child_idx = self.nodes_used;
        let right_child_idx = self.nodes_used + 1;
        self.nodes_used += 2;

        let node_first_prim = node.first_tri;
        let node_prim_count = node.tri_count;

        node.left_child = left_child_idx;
        node.tri_count = 0;

        self.nodes.push(BvhNode::new(node_first_prim, left_count, &self.triangles));
        self.nodes.push(BvhNode::new(i, node_prim_count - left_count, &self.triangles));

        self.subdivide(left_child_idx);
        self.subdivide(right_child_idx);
    }
}

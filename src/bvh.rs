use crate::{
    primitive::{Aabb, Ray},
    triangle::{Triangle, HitRecord}
};

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
    first_tri: usize,
    tri_count: usize
}

impl BvhNode {
    pub fn new(first_prim: usize, prim_count: usize, triangles: &Vec<Triangle>) -> Self {
        let mut aabb = Aabb::MAX;

        for tri in &triangles[first_prim..first_prim+prim_count] {
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
            first_tri: first_prim,
            tri_count: prim_count
        }
    }

    pub fn is_leaf(&self) -> bool {
        self.tri_count > 0
    }

    pub fn evaluate_sh(&self, axis: usize, pos: f32, triangles: &Vec<Triangle>) -> f32 {
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
        let mut node = &self.nodes[ROOT_IDX];
        let mut stack = [node; 64];
        let mut stack_pointer = 0;

        let mut nearest_dist = f32::INFINITY;
        let mut nearest_hit: Option<HitRecord> = None;

        loop {
            if node.is_leaf() {
                for triangle in &self.triangles[node.first_tri..node.first_tri+node.tri_count] {
                    match triangle.hit(&ray) {
                        Some(hit_record) if hit_record.t < nearest_dist => {
                            nearest_dist = hit_record.t;
                            nearest_hit = Some(hit_record);
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

            let child1 = &self.nodes[node.left_child];
            let child2 = &self.nodes[node.left_child + 1];

            let dist1 = child1.aabb.hit(ray).and_then(|d| (d < nearest_dist).then_some(d));
            let dist2 = child2.aabb.hit(ray).and_then(|d| (d < nearest_dist).then_some(d));

            let ((near_dist, near_child), (far_dist, far_child)) = if dist1.unwrap_or(f32::INFINITY) > dist2.unwrap_or(f32::INFINITY) {
                ((dist2, child2), (dist1, child1))
            } else {
                ((dist1, child1), (dist2, child2))
            };

            if near_dist.is_none() {
                if stack_pointer == 0 {
                    break;
                } else {
                    stack_pointer -= 1;
                    node = stack[stack_pointer];
                }
            } else {
                node = near_child;
                if far_dist.is_some() {
                    stack[stack_pointer] = far_child;
                    stack_pointer += 1;
                }
            }
        }

        return nearest_hit;
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

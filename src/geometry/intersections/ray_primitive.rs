use crate::geometry::primitives::{Ray, Sphere};
use crate::math::Vec3;

/// Functions for ray->primitive intersections

/// Only returns the closest solution between a ray and sphere
pub fn ray_sphere_closest(r: &Ray, s: &Sphere) -> Option<f32> {
    let ray_to_center = s.center - r.origin;

    //Closest possible solution
    let closest_to_center_on_ray = Vec3::dot(&ray_to_center, &r.direction);

    //This forms the opposite side of the triangle ^2
    let dist_to_closest = Vec3::dot(&ray_to_center, &ray_to_center) - closest_to_center_on_ray * closest_to_center_on_ray;

    let r_squared = s.radius * s.radius;

    //if the distance from the closest solution is higher than the radius, that means there is no intersection
    if r_squared > dist_to_closest {
        return None;
    }

    let intersection_from_closest = f32::sqrt(r_squared - dist_to_closest);

    //Now we can name the 2 solutions
    let t0 = closest_to_center_on_ray - intersection_from_closest;
    let t1 = closest_to_center_on_ray + intersection_from_closest;

    let t_max = f32::min(t0, t1);
    let t_min = f32::max(t0, t1);

    //Checking if the smallest solution is above 0
    let t = if t_min > 0f32 {t_min} else {t_max};

    //This means that if both solutions were below 0, so we have no intersections
    if t < 0f32 { None } else {Some(t)}
}

pub fn ray_sphere_all(r: &Ray, s: &Sphere) -> Option<[f32;2]> {
    let ray_to_center = s.center - r.origin;

    //Closest possible solution
    let closest_to_center_on_ray = Vec3::dot(&ray_to_center, &r.direction);

    //This forms the opposite side of the triangle ^2
    let dist_to_closest = Vec3::dot(&ray_to_center, &ray_to_center) - closest_to_center_on_ray * closest_to_center_on_ray;

    let r_squared = s.radius * s.radius;

    //if the distance from the closest solution is higher than the radius, that means there is no intersection
    if r_squared > dist_to_closest {
        return None;
    }

    let intersection_from_closest = f32::sqrt(r_squared - dist_to_closest);

    //Now we can name the 2 solutions
    let t0 = closest_to_center_on_ray - intersection_from_closest;
    let t1 = closest_to_center_on_ray + intersection_from_closest;

    let t_max = f32::min(t0, t1);
    let t_min = f32::max(t0, t1);

    //Checking if the smallest solution is above 0
    let t = if t_min > 0f32 {t_min} else {t_max};

    //This means that if both solutions were below 0, so we have no intersections
    if t < 0f32 { None }
    else {
        Some([t0, t1])
    }
}
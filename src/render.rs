use crate::*;

pub fn lighting(material: &Material, point_light: &PointLight, pos: &Tuples, eye_v: &Tuples, normal_v: &Tuples, in_shadow: bool) -> Tuples {
    let eff_color = material.pattern.color_at(pos).clone().multiply(point_light.intensity());
    let mut ambient = eff_color.clone().scale(material.ambient);
    if in_shadow {
        return ambient;
    }
    
    let light_v = Tuples::normalize(&mut point_light.position().clone().subtract(&pos));
    let light_dot_normal = Tuples::dot(&light_v, &normal_v);
    if light_dot_normal < 0.0 {
        return ambient;
    } else {
        let diffuse =  eff_color.clone().scale(material.diffuse).scale(light_dot_normal);
        let reflect_v = Tuples::reflect(&light_v.clone().negate(), &normal_v);
        let reflect_dot_eye = Tuples::dot(&reflect_v, &eye_v);
        let specular = if reflect_dot_eye <= 0.0 {
            Tuples::color(0.0,0.0,0.0)
        } else {
            let factor = reflect_dot_eye.powf(material.shininess);
            point_light.intensity().clone().scale(material.specular).scale(factor)
        };
        return ambient.add(&diffuse).add(&specular);
    }
}



pub fn render(camera: &Camera, world: &World) -> Canvas {
    let mut canvas = Canvas::new(camera.h_size, camera.v_size);
    for y in 0..camera.v_size {
        for x in 0..camera.h_size {
            let ray = Camera::ray_for_pixel(camera, x, y);
            let color = World::color_at(world, &ray);
            canvas.write_pixel(x, y, &color);
        }
    }
    canvas
}
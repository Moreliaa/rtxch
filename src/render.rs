use crate::*;

pub fn lighting(material: &Material, point_light: &PointLight, pos: &Tuples, eye_v: &Tuples, normal_v: &Tuples) -> Tuples {
    let eff_color = material.color.clone().multiply(point_light.intensity());
    let light_v = Tuples::normalize(&mut point_light.position().clone().subtract(&pos));
    let mut ambient = eff_color.clone().scale(material.ambient);
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
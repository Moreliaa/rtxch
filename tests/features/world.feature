Feature: World

Scenario: Creating a world
  Given w ← world()
  Then w contains no objects
    And w has no light source

Scenario: The default world
  Given p ← point(-10, 10, -10)
    And c ← color(1, 1, 1)
    And light ← point_light(p, c)
    And s1 ← sphere() with: material.color(0.8, 1.0, 0.6) | material.diffuse(0.7) | material.specular(0.2) |
    And s2 ← sphere() with: transform | scaling(0.5, 0.5, 0.5) |
  When w ← default_world()
  Then w.light = light
    And w contains s1
    And w contains s2

Scenario: Intersect a world with a ray
  Given w ← default_world()
    And p ← point(0, 0, -5)
    And v ← vector(0, 0, 1)
    And r ← ray(p, v)
  When xs ← intersect_world(w, r)
  Then xs.count = 4
    And xs[0].t = 4
    And xs[1].t = 4.5
    And xs[2].t = 5.5
    And xs[3].t = 6

Scenario: Shading an intersection
  Given w ← default_world()
    And p ← point(0, 0, -5)
    And v ← vector(0, 0, 1)
    And r ← ray(p, v)
    And shape ← the first object in w
    And i ← intersection(4, shape)
  When comps ← prepare_computations(i, r)
    And c ← shade_hit(w, comps)
  Then comps.inside = false
    And c = color(0.38066, 0.47583, 0.2855)

Scenario: Shading an intersection from the inside
  Given w ← default_world()
    And pl ← point(0, 0.25, 0)
    And cl ← color(1, 1, 1)
    And light ← point_light(pl, cl)
    And w.light ← light
    And pr ← point(0, 0, 0)
    And vr ← vector(0, 0, 1)
    And r ← ray(pr, vr)
    And shape ← the second object in w
    And i ← intersection(0.5, shape)
  When comps ← prepare_computations(i, r)
    And c ← shade_hit(w, comps)
  Then comps.inside = true
    And c = color(0.90498, 0.90498, 0.90498)

Scenario: The color when a ray misses
  Given w ← default_world()
    And p ← point(0, 0, -5)
    And v ← vector(0, 1, 0)
    And r ← ray(p, v)
  When c ← color_at(w, r)
  Then c = color(0, 0, 0)

Scenario: The color when a ray hits
  Given w ← default_world()
    And p ← point(0, 0, -5)
    And v ← vector(0, 0, 1)
    And r ← ray(p, v)
  When c ← color_at(w, r)
  Then c = color(0.38066, 0.47583, 0.2855)

Scenario: The color with an intersection behind the ray
  Given w ← default_world()
    And outer ← the first object in w
    And outer.material.ambient ← 1
    And inner ← the second object in w
    And inner.material.ambient ← 1
    And p ← point(0, 0, 0.75)
    And v ← vector(0, 0, -1)
    And r ← ray(p, v)
  When c ← color_at(w, r)
  Then c = inner.material.color

Scenario: There is no shadow when nothing is collinear with point and light
  Given w ← default_world()
    And p ← point(0, 10, 0)
   Then is_shadowed(w, p) is false

Scenario: The shadow when an object is between the point and the light
  Given w ← default_world()
    And p ← point(10, -10, 10)
   Then is_shadowed(w, p) is true

Scenario: There is no shadow when an object is behind the light
  Given w ← default_world()
    And p ← point(-20, 20, -20)
   Then is_shadowed(w, p) is false

Scenario: There is no shadow when an object is behind the point
  Given w ← default_world()
    And p ← point(-2, 2, -2)
   Then is_shadowed(w, p) is false

Scenario: shade_hit() is given an intersection in shadow
  Given w ← world()
    And lightp ← point(0, 0, -10)
    And lightc ← color(1, 1, 1)
    And light ← point_light(lightp, lightc)
    And w.light ← light
    And s1 ← sphere()
    And s1 is added to w
    And s2 ← sphere() with: | transform | translation(0, 0, 10) |
    And s2 is added to w
    And rayp ← point(0, 0, 5)
    And rayv ← vector(0, 0, 1)
    And r ← ray(rayp, rayv)
    And i ← intersection(4, s2)
  When comps ← prepare_computations(i, r)
    And c ← shade_hit(w, comps)
  Then c = color(0.1, 0.1, 0.1)

Scenario: The reflected color for a nonreflective material
  Given w ← default_world()
    And rayp ← point(0, 0, 0)
    And rayv ← vector(0, 0, 1)
    And r ← ray(rayp, rayv)
    And shape ← the second object in w
    And shape.material.ambient ← 1
    And i ← intersection(1, shape)
  When comps ← prepare_computations(i, r)
    And color ← reflected_color(w, comps)
  Then color = color(0, 0, 0)

Scenario: The reflected color for a reflective material
  Given w ← default_world()
    And shape ← plane() with:                 
      | material.reflective | 0.5                   |
      | transform           | translation(0, -1, 0) |   
    And shape is added to w
    And rayp ← point(0, 0, -3)
    And rayv ← vector(0, -0.70711, 0.70711)
    And r ← ray(rayp, rayv)
    And i ← intersection(1.41421, shape)
  When comps ← prepare_computations(i, r)
    And color ← reflected_color(w, comps)
  Then color = color(0.19032, 0.2379, 0.14274)

Scenario: shade_hit() with a reflective material
  Given w ← default_world()
    And shape ← plane() with:
      | material.reflective | 0.5                   |
      | transform           | translation(0, -1, 0) |
    And shape is added to w
    And rayp ← point(0, 0, -3)
    And rayv ← vector(0, -0.70711, 0.70711)
    And r ← ray(rayp, rayv)
    And i ← intersection(1.41421, shape)
  When comps ← prepare_computations(i, r)
    And color ← shade_hit(w, comps)
  Then color = color(0.87677, 0.92436, 0.82918)

Scenario: color_at() with mutually reflective surfaces
  Given w ← world()
    And p ← point(0, 0, 0)
    And c ← color(1, 1, 1)
    And w.light ← point_light(p, c)
    And lower ← plane() with:
      | material.reflective | 1                     |
      | transform           | translation(0, -1, 0) |
    And lower is added to w
    And upper ← plane() with:
      | material.reflective | 1                    |
      | transform           | translation(0, 1, 0) |
    And upper is added to w
    And rayp ← point(0, 0, 0)
    And rayv ← vector(0, 1, 0)
    And r ← ray(rayp, rayv)
  Then color_at(w, r) should terminate successfully

Scenario: The reflected color at the maximum recursive depth
  Given w ← default_world()
    And shape ← plane() with:
      | material.reflective | 0.5                   |
      | transform           | translation(0, -1, 0) |
    And shape is added to w
    And rayp ← point(0, 0, -3)
    And rayv ← vector(0, -0.70711, 0.70711)
    And r ← ray(rayp, rayv)
    And i ← intersection(1.41421, shape)
  When comps ← prepare_computations(i, r)
    And color ← reflected_color(w, comps, 0)    
  Then color = color(0, 0, 0)

Scenario: The refracted color with an opaque surface
  Given w ← default_world()
    And shape ← the first object in w
    And rayp ← point(0, 0, -5)
    And rayv ← vector(0, 0, 1)
    And r ← ray(rayp, rayv)
    And i1 ← intersection(4, shape)
    And i2 ← intersection(6, shape)
    And xs ← intersections(i1, i2)
  When comps ← prepare_computations(i1, r, xs)
    And c ← refracted_color(w, comps, 5)
  Then c = color(0, 0, 0)

Scenario: The refracted color at the maximum recursive depth
  Given w ← default_world()
    And shape ← the first object in w
    And shape has:
      | material.transparency     | 1.0 |
      | material.refractive_index | 1.5 |
    And rayp ← point(0, 0, -5)
    And rayv ← vector(0, 0, 1)
    And r ← ray(rayp, rayv)
    And i1 ← intersection(4, shape)
    And i2 ← intersection(6, shape)
    And xs ← intersections(i1, i2)
  When comps ← prepare_computations(i1, r, xs)
    And c ← refracted_color(w, comps, 0)
  Then c = color(0, 0, 0)

Scenario: The refracted color under total internal reflection
  Given w ← default_world()
    And shape ← the first object in w
    And shape has:
      | material.transparency     | 1.0 |
      | material.refractive_index | 1.5 |
    And rayp ← point(0, 0, 0.70711)
    And rayv ← vector(0, 1, 0)
    And r ← ray(rayp, rayv)
    And i1 ← intersection(-0.70711, shape)
    And i2 ← intersection(0.70711, shape)
    And xs ← intersections(i1, i2)
  # NOTE: this time you're inside the sphere, so you need
  # to look at the second intersection, xs[1], not xs[0]
  When comps ← prepare_computations(i2, r, xs)
    And c ← refracted_color(w, comps, 5)
  Then c = color(0, 0, 0)

Scenario: The refracted color with a refracted ray
  Given w ← default_world()
    And A ← the first object in w
    And A has:
      | material.ambient | 1.0            |
      | material.pattern | test_pattern() |
    And B ← the second object in w
    And B has:
      | material.transparency     | 1.0 |
      | material.refractive_index | 1.5 |
    And rayp ← point(0, 0, 0.1)
    And rayv ← vector(0, 1, 0)
    And r ← ray(rayp, rayv)
    And i1 ← intersection(-0.9899, A)
    And i2 ← intersection(-0.4899, B)
    And i3 ← intersection(0.4899, B)
    And i4 ← intersection(0.9899, A)
    And xs ← intersections(i1, i2, i3, i4)
  When comps ← prepare_computations(i3, r, xs)
    And c ← refracted_color(w, comps, 5)
  Then c = color(0, 0.99888, 0.04725)

Scenario: shade_hit() with a transparent material
  Given w ← default_world()
    And floor ← plane() with:
      | transform                 | translation(0, -1, 0) |
      | material.transparency     | 0.5                   |
      | material.refractive_index | 1.5                   |
    And floor is added to w
    And ball ← sphere() with:
      | material.color     | (1, 0, 0)                  |
      | material.ambient   | 0.5                        |
      | transform          | translation(0, -3.5, -0.5) |
    And ball is added to w
    And rp ← point(0, 0, -3)
    And rv ← vector(0, -0.70711, 0.70711)
    And r ← ray(rp, rv)
    And i0 ← intersection(1.41421, floor)
    And xs ← intersections(i0)
  When comps ← prepare_computations(i0, r, xs)
    And color ← shade_hit(w, comps, 5)
  Then color = color(0.93642, 0.68642, 0.68642)

Scenario: shade_hit() with a reflective, transparent material
  Given w ← default_world()
    And rp ← point(0, 0, -3)
    And rv ← vector(0, -0.70711, 0.70711)
    And r ← ray(rp, rv)
    And floor ← plane() with:
      | transform                 | translation(0, -1, 0) |
      | material.reflective       | 0.5                   |
      | material.transparency     | 0.5                   |
      | material.refractive_index | 1.5                   |
    And floor is added to w
    And ball ← sphere() with:
      | material.color     | (1, 0, 0)                  |
      | material.ambient   | 0.5                        |
      | transform          | translation(0, -3.5, -0.5) |
    And ball is added to w
    And i0 ← intersection(1.41421, floor)
    And xs ← intersections(i0)
  When comps ← prepare_computations(i0, r, xs)
    And color ← shade_hit(w, comps, 5)
  Then color = color(0.93391, 0.69643, 0.69243)

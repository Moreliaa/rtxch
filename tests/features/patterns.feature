Feature: Patterns

Background:
  Given black ← color(0, 0, 0)
    And white ← color(1, 1, 1)

Scenario: Creating a stripe pattern
  Given pattern ← stripe_pattern(white, black)
  Then pattern.a = white
    And pattern.b = black

Scenario: A stripe pattern is constant in y
  Given pattern ← stripe_pattern(white, black)
  Then color_at(pattern, point(0, 0, 0)) = white
    And color_at(pattern, point(0, 1, 0)) = white
    And color_at(pattern, point(0, 2, 0)) = white

Scenario: A stripe pattern is constant in z
  Given pattern ← stripe_pattern(white, black)
  Then color_at(pattern, point(0, 0, 0)) = white
    And color_at(pattern, point(0, 0, 1)) = white
    And color_at(pattern, point(0, 0, 2)) = white

Scenario: A stripe pattern alternates in x
  Given pattern ← stripe_pattern(white, black)
  Then color_at(pattern, point(0, 0, 0)) = white
    And color_at(pattern, point(0.9, 0, 0)) = white
    And color_at(pattern, point(1, 0, 0)) = black
    And color_at(pattern, point(-0.1, 0, 0)) = black
    And color_at(pattern, point(-1, 0, 0)) = black
    And color_at(pattern, point(-1.1, 0, 0)) = white

Scenario: Stripes with an object transformation
  Given object ← sphere()
    And scale ← scaling(2, 2, 2)
    And set_transform(object, scale)
    And pattern ← stripe_pattern(white, black)
  Then color_at_object(pattern, object, point(1.5, 0, 0)) = white

Scenario: Stripes with a pattern transformation
  Given object ← sphere()
    And scale ← scaling(2, 2, 2)
    And pattern ← stripe_pattern(white, black)
    And set_pattern_transform(pattern, scale)
  Then color_at_object(pattern, object, point(1.5, 0, 0)) = white

Scenario: Stripes with both an object and a pattern transformation
  Given object ← sphere()
    And scale ← scaling(2, 2, 2)
    And set_transform(object, scale)
    And pattern ← stripe_pattern(white, black)
    And trans ← translation(0.5, 0, 0)
    And set_pattern_transform(pattern, trans)
  Then color_at_object(pattern, object, point(2.5, 0, 0)) = white

Scenario: A gradient linearly interpolates between colors
  Given pattern ← gradient_pattern(white, black)
    And c1 ← color(0.75, 0.75, 0.75)
    And c2 ← color(0.5, 0.5, 0.5)
    And c3 ← color(0.25, 0.25, 0.25)
  Then color_at(pattern, point(0, 0, 0)) = white
    And color_at(pattern, point(0.25, 0, 0)) = c1
    And color_at(pattern, point(0.5, 0, 0)) = c2
    And color_at(pattern, point(0.75, 0, 0)) = c3
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

Scenario: A ring should extend in both x and z
  Given pattern ← ring_pattern(white, black)
  Then color_at(pattern, point(0, 0, 0)) = white
    And color_at(pattern, point(1, 0, 0)) = black
    And color_at(pattern, point(0, 0, 1)) = black
    # 0.708 = just slightly more than √2/2
    And color_at(pattern, point(0.708, 0, 0.708)) = black

Scenario: The default pattern transformation
  Given pattern ← test_pattern(white, black)
  Then pattern.transform = identity_matrix

Scenario: Assigning a transformation
  Given pattern ← test_pattern(white, black)
    And trans ← translation(1, 2, 3)
  When set_pattern_transform(pattern, trans)
  Then pattern.transform = trans

Scenario: A pattern with an object transformation
  Given shape ← sphere()
    And scale ← scaling(2, 2, 2)
    And set_transform(shape, scale)
    And pattern ← test_pattern(white, black)
    And c ← color(1, 1.5, 2)
  Then color_at_object(pattern, shape, point(2, 3, 4)) = c

Scenario: A pattern with a pattern transformation
  Given shape ← sphere()
    And scale ← scaling(2, 2, 2)
    And pattern ← test_pattern(white, black)
    And set_pattern_transform(pattern, scale)
    And c ← color(1, 1.5, 2)
  Then color_at_object(pattern, shape, point(2, 3, 4)) = c

Scenario: A pattern with both an object and a pattern transformation
  Given shape ← sphere()
    And scale ← scaling(2, 2, 2)
    And set_transform(shape, scale)
    And pattern ← test_pattern(white, black)
    And trans ← translation(0.5, 1, 1.5)
    And set_pattern_transform(pattern, trans)
    And c ← color(0.75, 0.5, 0.25)
  Then color_at_object(pattern, shape, point(2.5, 3, 3.5)) = c

Scenario: Checkers should repeat in x
  Given pattern ← checkers_pattern(white, black)
  Then color_at(pattern, point(0, 0, 0)) = white
    And color_at(pattern, point(0.99, 0, 0)) = white
    And color_at(pattern, point(1.01, 0, 0)) = black

Scenario: Checkers should repeat in y
  Given pattern ← checkers_pattern(white, black)
  Then color_at(pattern, point(0, 0, 0)) = white
    And color_at(pattern, point(0, 0.99, 0)) = white
    And color_at(pattern, point(0, 1.01, 0)) = black

Scenario: Checkers should repeat in z
  Given pattern ← checkers_pattern(white, black)
  Then color_at(pattern, point(0, 0, 0)) = white
    And color_at(pattern, point(0, 0, 0.99)) = white
    And color_at(pattern, point(0, 0, 1.01)) = black

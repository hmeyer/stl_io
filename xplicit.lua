cube = Cube(1,1,1,0.3)
sphere = Sphere(0.5)
diff = Difference({cube, sphere}, 0.3)
diff:scale(15,15,15)

build(diff)
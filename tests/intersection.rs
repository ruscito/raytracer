use raytracer::{intersection::{Intersection, Intersections}, shape::Shape, shape::Sphere};

#[test]
fn create_intersection() {
    let s = Box::new(Sphere::new());
    let i = Intersection::new(3.5, s.clone());
    assert_eq!(i.t, 3.5);
    assert_eq!(i.object.id(), s.id());
}

#[test]
#[should_panic]
fn eq_intersections() {
    let s = Box::new(Sphere::new());
    let i1 = Intersection::new(1.0, s.clone());
    let i2 = Intersection::new(1.0, s.clone());    
    if i1 == i2 {
        panic!("good")
    }
}

#[test]
fn aggregating_intrsections() {
    let s = Box::new(Sphere::new());
    let i1 = Intersection::new(1.0, s.clone());
    let i2 = Intersection::new(2.0, s.clone());
    let xs = Intersections::new(vec![i2, i1]);
    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0].t, 1.0);
    assert_eq!(xs[1].t, 2.0);
}

#[test]
fn hit_1(){
    let s = Box::new(Sphere::new());
    let i1 = Intersection::new(1.0, s.clone());
    let i2 = Intersection::new(2.0, s);
    let xs = Intersections::new(vec![i2, i1.clone()]);
    let i = xs.hit().unwrap();
    assert_eq!(i, i1);
}

#[test]
fn hit_2(){
    let s = Box::new(Sphere::new());
    let i1 = Intersection::new(-1.0, s.clone());
    let i2 = Intersection::new(1.0, s);
    let xs = Intersections::new(vec![i2.clone(), i1]);
    let i = xs.hit().unwrap();
    assert_eq!(i, i2);
}

#[test]
#[should_panic]
fn hit_3(){
    let s = Box::new(Sphere::new());
    let i1 = Intersection::new(-2.0, s.clone());
    let i2 = Intersection::new(-1.0, s);
    let xs = Intersections::new(vec![i1, i2]);
    let _i = xs.hit().unwrap();
}

#[test]
#[should_panic]
fn hit_4(){
    let s = Box::new(Sphere::new());
    let i1 = Intersection::new(5.0, s.clone());
    let i2 = Intersection::new(7.0, s.clone());
    let i3 = Intersection::new(-3.0, s.clone());
    let i4 = Intersection::new(-2.0, s);
    let xs = Intersections::new(vec![i1, i2, i3, i4.clone()]);
    let i = xs.hit().unwrap();
    assert_eq!(i, i4);
}

module align(x,y,z) rotate(atan2(sqrt(x^2+y^2),z),[-y,x,0]) children();
module socket(h, chans=7) union() {
  cylinder(h,r=1.6,$fn=50);
  for (a=[0:chans]) rotate(a/chans*360) translate([1.6,0,0]) cylinder(h,d=1,$fn=50);
}
module vertex(vs, bite=5) difference() {
  union() { sphere(4, $fn=50); for(v=vs) align(v[0],v[1],v[2]) cylinder(bite+2.4,r=3,$fn=50); }
  sphere(3,$fn=50); for(v=vs) align(v[0],v[1],v[2]) socket(bite+2.5);
}
zilch=.00000000000001; // TODO: fix `vertex([[0,0,-1]])` so zilch isn't needed

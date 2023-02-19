use rtbvh::{Bvh, Builder};

use crate::lib::{shape::Shape, tool::{bound::Bound3, interaction::Interaction}, ray::Ray};
use rtbvh::Primitive as OtherPrimitive;

use super::{Aggregate, Primitive as MyPrimitive};
#[allow(clippy::upper_case_acronyms)]

pub struct BVH{
    bvh:Bvh,
    shapes:Vec<Shape>,
}
impl BVH{
    pub fn build(shapes:Vec<Shape>)->Self{
        let aabbs=shapes.iter().map(|t|{
            t.aabb()
        }).collect::<Vec<_>>();
        let builder=Builder{
            aabbs:Some(aabbs.leak()),
            primitives:shapes.as_slice(),
            primitives_per_leaf: std::num::NonZeroUsize::new(shapes.len()*2)
        };
        let bvh = builder.construct_binned_sah().expect("加速结构失败");
        Self{
            bvh,
            shapes,
        }
    }
}
impl Aggregate for BVH{
    fn world_bound(&self) -> crate::lib::tool::bound::Bound3 {
        self.shapes.iter().fold(Bound3::default(), |a,b|{
            a.merage(&b.world_bound())
        })
    }

    fn interacect(&self, ray: &crate::lib::ray::Ray) -> Option<crate::lib::tool::interaction::Interaction> {
        let mut a=ray.to_rtbvh();
        let iter=self.bvh.traverse_iter(&mut a, &self.shapes);
        let mut ans:Option<Interaction>=None;
        for (shape,ray) in iter{
            let myray=Ray::new(ray.origin,ray.direction);
            if let Some(v)=shape.interacect(&myray){
                if let Some(ref item) = ans {
                    if item.time>ray.t{
                        ans=Some(v);
                    }
                }else{
                    ans=Some(v)
                }
            }
        };
        ans

    }
}

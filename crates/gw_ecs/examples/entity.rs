use gw_ecs::entities::{Entities, EntityMutSet, EntitySet};

struct Info(u32);

fn main() {
    let mut entities = Entities::default();

    let id = {
        let mut entry = entities.create();
        entry.insert(Info(5));
        entry.id()
    };

    {
        let entry = entities.get(id).unwrap();
        println!("Info = {}", entry.get::<Info>().unwrap().0);
    }

    for entity in entities.iter() {
        println!("Iter: Info = {}", entity.get::<Info>().unwrap().0);
    }

    let id_b = {
        let mut entry = entities.create();
        entry.insert(Info(6));
        entry.id()
    };

    {
        let a = id.fetch(&entities);
        let info = a.get::<Info>().unwrap();
        println!("Via EntitySet(0): {}", info.0);
    }

    {
        let (mut a, b) = (id, id_b).fetch_mut(&mut entities);

        let mut info_a = a.get_mut::<Info>().unwrap();
        info_a.0 = info_a.0 + 1;
        let info_b = b.get::<Info>().unwrap();
        println!("Via EntitySet - Mut(2): {} - {}", info_a.0, info_b.0);
    }

    {
        let (a, b) = (id, id_b).fetch(&entities);

        let info_a = a.get::<Info>().unwrap();
        let info_b = b.get::<Info>().unwrap();
        println!("Via EntitySet(2): {} - {}", info_a.0, info_b.0);
    }
}

use super::NoiseMap;

pub fn gen(width: usize, height: usize) -> NoiseMap {
    let mut map = NoiseMap::new(width, height);

    for x in 0..map.width() {
        for y in 0..map.height() {
            map[x][y] = rand::random::<u8>()
        }
    }

    return map;
}

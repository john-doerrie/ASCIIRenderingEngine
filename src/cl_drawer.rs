use crate::cl_map::ClMap;

pub(crate) struct ClDrawer {
    map: ClMap,

}

impl ClDrawer {
    pub(crate) fn new(width: usize, height: usize) -> ClDrawer {
        ClDrawer {
            map: ClMap::new(width, height)
        }
    }
    
    fn draw(&self) {
        
    }
}
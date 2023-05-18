use crate::cell::Cell;
use crate::common::Direction;
use crate::state::State;

/// ただのセルの集合。Stateと位置情報の集合体。
/// 軽量化...?固定値、Vecを使わない...?
struct Field3d {
	field: [[[Cell; 10];10];10],
	
}
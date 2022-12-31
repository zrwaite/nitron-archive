# Graphics ideas

can graphics take complete control of the game entities in every loop?
If so, with positions, should it propagate up or down the parent/child tree?

enum UIElementType {
	Box(UIBox)
	Image(UIImage)
	Text(UIText)
}

struct UIElement {
	x: u32,
	y: u32,
	w: i32,
	h: i32,
	pub on_mousein: ||=>{},
	pub on_mouseout: ||=>{},
	pub on_click: ||=>{},
	rotation: u16
	children_ids: Vec<String> // one or the other
	parent_id: Option<String>
	position: RELATIVE|ABSOLUTEs
	display: bool

	pub set_w: fn
	pub set_h: fn
	pub set_x: fn
	pub set_y: fn 
	pub set_rotation: fn
	move: fn 
	set_display: fn

	type: UIElementType
}


```

UIBox {
	border_radius
	pub color: Color,
	pub padding: u32,
	pub border_color: Color
	// traits to propagate fields up to top level
}

UIImage {
	texture_id: String
	// traits to propagate fields up to top level
}

UIText {
	text: String
	// traits to propagate fields up to top level
}
```
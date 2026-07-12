pub struct Tree<'a> {
    pub head: Node<'a>,
}

pub struct Node<'a> {
    pub component: crate::component::Component<'a>,
    pub children: Vec<Node<'a>>,
    pub clips: bool,
}

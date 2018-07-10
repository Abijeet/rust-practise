struct Node {
  children: Vec<Node>,
  node_type: NodeType
}

enum NodeType {
  Text(String),
  Element(ElementData)
}

struct ElementData {
  tag_name: String,
  attributes: AttrMap
}

type AttrMap = HashMap<String, String>;

fn text(data: String) {
  Node { children: Vec::new(), node_type: NodeType::Text(data) }
}

fn element(name: String, attrs: AttrMap, children: Vec<Node>) -> Node {
  Node {
    children: children,
    node_type: NodeType::Element(ElementData {
      tag_name: name,
      attributes: attrs
    })
  }
}
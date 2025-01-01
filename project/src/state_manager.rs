#[derive(Debug)]
enum States{
    Idle,
    Dragging(Node),
    Linking(Node),
    Editing,
}
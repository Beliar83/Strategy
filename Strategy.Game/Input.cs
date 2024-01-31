namespace Strategy.Game;

public class Input
{
    public record struct Button
    {
        public record struct Select;
        public record struct Cancel;
    }

    public record struct CursorMoved(Hexagon CursorCell);

    public record struct ButtonPressed(Button Button);
}


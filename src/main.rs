use pretty::termcolor::{Color, ColorChoice, ColorSpec, StandardStream};
use pretty::{Arena, DocAllocator};

fn main() {
    let arena = Arena::new();
    let one = arena
        .text("This crate provides functionality for defining pretty printers.")
        .annotate(ColorSpec::new().set_fg(Some(Color::Red)).clone());

    let two = arena
        .text("It is particularly useful for printing structured recursive data like trees.")
        .annotate(ColorSpec::new().set_fg(Some(Color::White)).clone());

    let three = arena
        .text("The implementation was originally based on Larsen's SML translation")
        .annotate(ColorSpec::new().set_fg(Some(Color::Blue)).clone());

    let four = arena
        .text("(https://github.com/kfl/wpp)")
        .annotate(ColorSpec::new().set_bold(true).clone());

    let five = arena
        .text("of Wadler's Haskell pretty printer")
        .annotate(ColorSpec::new().set_intense(true).clone());

    one.append(arena.space())
        .append(
            two.append(arena.space())
                .append(three)
                .append(arena.space())
                .append(four)
                .append(arena.space())
                .append(five)
                .nest(1),
        )
        .1
        .render_colored(80, StandardStream::stdout(ColorChoice::Auto))
        .unwrap();
}

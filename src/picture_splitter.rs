// This is responsible for splitting a picture resource into strongly typed actions+args.
// https://www.agidev.com/articles/agispec/agispecs-7.html

#[derive(PartialEq, Debug)]
pub enum Action {
    ChangePictureColourAndEnablePictureDraw, // 0xF0
    DisablePictureDraw, // 0xF1
    ChangePriorityColourAndEnablePriorityDraw, // 0xF2
    DisablePriorityDraw, // 0xF3
    DrawAYCorner, // 0xF4
    DrawAnXCorner, // 0xF5
    AbsoluteLines, // 0xF6
    RelativeLines, // 0xF7
    Fill, // 0xF8
    ChangePenSizeAndStyle, // 0xF9
    PlotWithPen, // 0xFA
    Unused, // 0xFB--0xFE
    End, // 0xFF
}

#[derive(PartialEq, Debug)]
pub struct ActionArguments {
    pub action: Action,
    pub arguments: Vec<u8>,
}

pub fn split(data: &[u8]) -> Vec<ActionArguments> {
    let mut last_action: Option<Action> = None;
    let mut last_args: Vec<u8> = Vec::new();
    let mut actions: Vec<ActionArguments> = Vec::new();
    for &b in data {
        if b >= 0xf0 { // New action.
            if let Some(action) = last_action {
                actions.push(ActionArguments {
                    action,
                    arguments: last_args.clone(),
                });
            }
            last_action = Some(Action::from_byte(b));
            last_args.clear();
        } else {
            last_args.push(b);
        }
    }
    if let Some(action) = last_action {
        actions.push(ActionArguments {
            action,
            arguments: last_args.clone(),
        });
    }
    actions
}

impl Action {
    fn from_byte(b: u8) -> Self {
        match b {
            0xF0 => Self::ChangePictureColourAndEnablePictureDraw,
            0xF1 => Self::DisablePictureDraw,
            0xF2 => Self::ChangePriorityColourAndEnablePriorityDraw,
            0xF3 => Self::DisablePriorityDraw,
            0xF4 => Self::DrawAYCorner,
            0xF5 => Self::DrawAnXCorner,
            0xF6 => Self::AbsoluteLines,
            0xF7 => Self::RelativeLines,
            0xF8 => Self::Fill,
            0xF9 => Self::ChangePenSizeAndStyle,
            0xFA => Self::PlotWithPen,
            0xFF => Self::End,
            _ => Self::Unused,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_splits() {
        let resource: Vec<u8> = vec![
            1, // Should be ignored.
            0xf0, 1, 2, 3,
            0xf1,
            0xf2,
            0xff,
        ];
        let result = super::split(&resource);
        let expected: Vec<super::ActionArguments> = vec![
            ActionArguments{
                action: Action::ChangePictureColourAndEnablePictureDraw,
                arguments: vec![1, 2, 3],
            },
            ActionArguments{
                action: Action::DisablePictureDraw,
                arguments: vec![],
            },
            ActionArguments{
                action: Action::ChangePriorityColourAndEnablePriorityDraw,
                arguments: vec![],
            },
            ActionArguments{
                action: Action::End,
                arguments: vec![],
            },
        ];
        assert_eq!(result, expected);
    }
}

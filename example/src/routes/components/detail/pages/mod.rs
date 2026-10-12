use super::super::catalog::component_by_slug;
use super::page::ComponentPage;

pub fn component_page(slug: &str) -> Option<ComponentPage> {
    let spec = component_by_slug(slug)?;

    Some(match slug {
        "accordion" => accordion::page(spec),
        "alert" => alert::page(spec),
        "appbar" => appbar::page(spec),
        "autocomplete" => autocomplete::page(spec),
        "avatar" => avatar::page(spec),
        "badge" => badge::page(spec),
        "bottom-navigation" => bottom_navigation::page(spec),
        "bottom-sheet" => bottom_sheet::page(spec),
        "breadcrumbs" => breadcrumbs::page(spec),
        "button" => button::page(spec),
        "card" => card::page(spec),
        "cascader" => cascader::page(spec),
        "checkbox" => checkbox::page(spec),
        "chip" => chip::page(spec),
        "circle-menu" => circle_menu::page(spec),
        "code-block" => code_block::page(spec),
        "code-engine" => code_engine::page(spec),
        "collapse" => collapse::page(spec),
        "datepicker" => datepicker::page(spec),
        "dialog" => dialog::page(spec),
        "divider" => divider::page(spec),
        "dm-markdown" => dm_markdown::page(spec),
        "drawer" => drawer::page(spec),
        "file-upload" => file_upload::page(spec),
        "form" => form::page(spec),
        "form-group" => form_group::page(spec),
        "grid" => grid::page(spec),
        "input" => input::page(spec),
        "list" => list::page(spec),
        "markdown-body" => markdown_body::page(spec),
        "markdown-input" => markdown_input::page(spec),
        "menu" => menu::page(spec),
        "modal" => modal::page(spec),
        "multi-select" => multi_select::page(spec),
        "navbar" => navbar::page(spec),
        "nested-menu" => nested_menu::page(spec),
        "otp-input" => otp_input::page(spec),
        "pagination" => pagination::page(spec),
        "pin-input" => pin_input::page(spec),
        "popover" => popover::page(spec),
        "progress" => progress::page(spec),
        "radio" => radio::page(spec),
        "rating" => rating::page(spec),
        "segment-control" => segment_control::page(spec),
        "select" => select::page(spec),
        "skeleton" => skeleton::page(spec),
        "slider" => slider::page(spec),
        "snackbar" => snackbar::page(spec),
        "stepper" => stepper::page(spec),
        "switch" => switch::page(spec),
        "table" => table::page(spec),
        "tabs" => tabs::page(spec),
        "textarea" => textarea::page(spec),
        "time-input" => time_input::page(spec),
        "timeline" => timeline::page(spec),
        "toast" => toast::page(spec),
        "toggle" => toggle::page(spec),
        "tooltip" => tooltip::page(spec),
        "tree-select" => tree_select::page(spec),
        "stat" => stat::page(spec),
        "kbd" => kbd::page(spec),
        "carousel" => carousel::page(spec),
        "diff" => diff::page(spec),
        "console-page" => console_page::page(spec),
        "footer" => footer::page(spec),
        "hero" => hero::page(spec),
        "home-page" => home_page::page(spec),
        "indicator" => indicator::page(spec),
        "join" => join::page(spec),
        "mask" => mask::page(spec),
        "sidebar-layout" => sidebar_layout::page(spec),
        "sign-page" => sign_page::page(spec),
        "stack" => stack::page(spec),
        "dropdown" => dropdown::page(spec),
        "fab" => fab::page(spec),
        "swap" => swap::page(spec),
        "megamenu" => megamenu::page(spec),
        "validator" => validator::page(spec),
        "range" => range::page(spec),
        "file-input" => file_input::page(spec),
        "filter-group" => filter_group::page(spec),
        "toggle-switch" => toggle_switch::page(spec),
        "loading" => loading::page(spec),
        "radial-progress" => radial_progress::page(spec),
        "countdown" => countdown::page(spec),
        "chat" => chat::page(spec),
        "theme-controller" => theme_controller::page(spec),
        "link" => link::page(spec),
        _ => return None,
    })
}

mod accordion;
mod alert;
mod appbar;
mod autocomplete;
mod avatar;
mod badge;
mod bottom_navigation;
mod bottom_sheet;
mod breadcrumbs;
mod button;
mod card;
mod carousel;
mod cascader;
mod chat;
mod checkbox;
mod chip;
mod circle_menu;
mod code_block;
mod code_engine;
mod collapse;
mod console_page;
mod countdown;
mod datepicker;
mod dialog;
mod diff;
mod divider;
mod dm_markdown;
mod drawer;
mod dropdown;
mod fab;
mod file_input;
mod file_upload;
mod filter_group;
mod footer;
mod form;
mod form_group;
mod grid;
mod hero;
mod home_page;
mod indicator;
mod input;
mod join;
mod kbd;
mod link;
mod list;
mod loading;
mod markdown_body;
mod markdown_input;
mod mask;
mod megamenu;
mod menu;
mod modal;
mod multi_select;
mod navbar;
mod nested_menu;
mod otp_input;
mod pagination;
mod pin_input;
mod popover;
mod progress;
mod radial_progress;
mod radio;
mod range;
mod rating;
mod segment_control;
mod select;
mod sidebar_layout;
mod sign_page;
mod skeleton;
mod slider;
mod snackbar;
mod stack;
mod stat;
mod stepper;
mod swap;
mod switch;
mod table;
mod tabs;
mod textarea;
mod theme_controller;
mod time_input;
mod timeline;
mod toast;
mod toggle;
mod toggle_switch;
mod tooltip;
mod tree_select;
mod validator;

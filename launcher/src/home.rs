//! Home in Material You 3 x One UI blur: glass hero over the background, pill controls,
//! squircle cards, accent primary colour derived from the player's hue.

use super::*;
use crate::m3_theme::{card, pill, M3Theme};
use crate::shell::{action, c, label, Action};
use gpui::prelude::*;

impl Launcher {
    pub(super) fn hero(
        &mut self,
        tag: &str,
        game: &str,
        prepared: bool,
        cx: &mut Context<Self>,
    ) -> Div {
        let t = self.theme;
        let m3 = M3Theme::for_accent(self.settings.accent, t.dark);
        let icon = self.selected_instance().cloned().map(|i| self.instance_icon(&i, 56.));
        let instance = self.selected_instance();
        let name = instance.map_or(
            if tag.is_empty() { "Minecraft" } else { "Horizon" },
            |i| i.name.as_str(),
        );
        let mut what = vec![format!("Minecraft: Java Edition {game}")];
        if !tag.is_empty() {
            what.push(format!(
                "Horizon {}",
                horizon_launcher::nightly::label(tag)
                    .unwrap_or_else(|| tag.trim_start_matches('v').to_string())
            ));
        }
        what.push(tr(if prepared { "Ready" } else { "Needs preparing" }).to_string());
        let playing = self.playing.map(|count| {
            let label = format!("· {}", trf("{count} playing now", &[("count", &thousands(count))]));
            match telemetry::stats_url(&self.links.website) {
                Some(url) => div().id("playing-now").cursor_pointer()
                    .hover(move |s| s.text_color(m3.m3.on_surface_variant))
                    .child(label).on_click(move |_: &ClickEvent, _, cx| cx.open_url(&url))
                    .into_any_element(),
                None => div().child(label).into_any_element(),
            }
        });
        let official = self.official_players.map(|count| {
            div().id("official-players").cursor_pointer()
                .hover(move |s| s.text_color(m3.m3.on_surface_variant))
                .child(format!("· {} on the official SMP", thousands(count)))
                .on_click(cx.listener(|this, _: &ClickEvent, _, cx| {
                    this.go(Page::Servers); cx.notify();
                }))
        });
        let state = div().flex().items_center().gap(px(8.))
            .child(div().flex_none().size(px(8.)).rounded(m3.shapes.small)
                .bg(if prepared { m3.m3.primary } else { m3.m3.warn }))
            .child(div().font_size(px(m3.type_.body_small)).font_weight(FontWeight::MEDIUM)
                .text_color(m3.m3.on_surface_variant).flex().gap(px(4.))
                .child(what.join(" · ")).children(playing).children(official));
        let mut hero = div().flex().flex_none().flex_col().gap(px(14.))
            .child(state)
            .child(div().flex().items_center().gap(px(16.)).children(icon)
                .child(div().font_size(px(m3.type_.headline_large)).font_weight(FontWeight::SEMIBOLD)
                    .text_color(m3.m3.on_surface).min_w_0().truncate().child(name.to_string())))
            .children(instance.map(|i| div().font_size(px(m3.type_.body_medium))
                .text_color(m3.m3.on_surface_variant).child(instances_page::played_line(i))));
        if let Some(release) = self.updates.horizon.as_ref().filter(|_| !tag.is_empty()) {
            let installing = self.installing.as_ref().filter(|(t2, _)| *t2 == release.tag);
            let line = div().flex().items_center().gap(px(8.))
                .child(div().size(px(8.)).rounded_full().bg(m3.m3.primary))
                .child(div().font_size(px(m3.type_.label_medium)).text_color(m3.m3.on_surface_variant)
                    .child(trf("{version} available", &[("version", update_name(release))])));
            hero = hero.child(match installing {
                Some((_, p)) => line.child(div().w(px(120.)).flex().child(progress_bar(Some(self.bar), &t)))
                    .child(format!("{:.0}%", p * 100.)),
                None => line.child(div().id("install-update").text_color(m3.m3.primary)
                    .font_weight(FontWeight::SEMIBOLD).cursor_pointer()
                    .hover(|s| s.opacity(0.8)).child(tr("Install"))
                    .on_click(cx.listener(|this, _: &ClickEvent, _, cx| {
                        this.install_update(); cx.notify();
                    }))),
            });
        }
        if !self.settings.ping_notice_seen && !self.collection.consent.needs_asking() {
            hero = hero.child(card(&m3).flex().flex_none().flex_col().gap(px(8.))
                .p(px(16.)).w_full().max_w(px(560.))
                .child(div().font_size(px(m3.type_.body_medium)).text_color(m3.m3.on_surface)
                    .child(tr(telemetry::PING_NAME)))
                .child(div().font_size(px(m3.type_.body_small)).text_color(m3.m3.on_surface_variant)
                    .child(tr(telemetry::PING)))
                .child(div().font_size(px(m3.type_.label_medium)).text_color(m3.m3.on_primary_container)
                    .font_weight(FontWeight::SEMIBOLD).cursor_pointer()
                    .hover(|s| s.opacity(0.8)).id("ping-notice-ok").child(tr("Got it"))
                    .on_click(cx.listener(|this, _: &ClickEvent, _, cx| {
                        this.settings.ping_notice_seen = true; this.settings.save(); cx.notify();
                    }))),
            );
        }
        hero
    }

    pub(super) fn home_actions(&self, ready: bool, java: bool, cx: &mut Context<Self>) -> Div {
        let t = self.theme;
        let m3 = M3Theme::for_accent(self.settings.accent, t.dark);
        let running = self.client.is_some() || self.starting_client.is_some();
        if self.preparing {
            let download = download_progress();
            let busy = match download {
                Some((done, total)) => format!("{} {:.0}%", tr("Getting ready…"), done as f32 * 100. / total as f32),
                None => tr("Getting ready…").to_string(),
            };
            return div().flex().items_center().gap(px(10.))
                .child(action("play", Action::Main, &t).min_w(px(260.)).cursor_default().child(label(busy)))
                .children(download.map(|_| action("download-cancel", Action::Quiet, &t)
                    .child(label(tr("Cancel").to_string()))
                    .on_click(|_, _, _| { horizon_launcher::mojang::CANCEL.store(true, Ordering::Relaxed) })));
        }
        let mut play = action("play", Action::Main, &t).min_w(px(180.))
            .child(div().relative().child(blocks::icon("play", 20., m3.m3.on_primary)))
            .child(label(tr(if running { "Running" } else { "Play" }).to_string()));
        match play_press(ready, self.owner, self.preparing || running) {
            PlayPress::Nothing => play = disabled(play),
            PlayPress::Play => play = play.on_click(cx.listener(|this, _: &ClickEvent, _, cx| {
                this.picker_open = false; this.play(); cx.notify();
            })),
            PlayPress::PrepareThenPlay => play = play.on_click(cx.listener(|this, _: &ClickEvent, _, cx| {
                this.picker_open = false; this.play_when_prepared = this.selected().cloned();
                this.prepare(backend::MinecraftFrom::Install); cx.notify();
            })),
        }
        let folder = self.selected_instance().map(|i| i.home.clone());
        div().flex().flex_wrap().items_center().gap(px(10.))
            .child(play).children((!java).then(|| self.picker(cx)))
            .child(action("home-mods", Action::Quiet, &t).child(label(tr("Mods").to_string()))
                .on_click(cx.listener(|this, _: &ClickEvent, _, cx| {
                    this.go(Page::Mods); cx.notify();
                })))
            .children(folder.map(|home| action("home-folder", Action::Quiet, &t)
                .child(label(tr("Folder").to_string()))
                .on_click(move |_, _, cx| { let _ = std::fs::create_dir_all(&home); cx.open_with_system(&home); })))
    }

    pub(super) fn home_card(&self, tag: &str, cx: &mut Context<Self>) -> Div {
        let t = self.theme;
        let m3 = M3Theme::for_accent(self.settings.accent, t.dark);
        let summary = match &self.releases {
            Load::Ready(list) => list.iter().find(|r| r.tag == tag)
                .filter(|r| r.nightly.is_none())
                .and_then(|r| backend::notes_summary(&r.body)),
            _ => None,
        };
        let heading = |s: &str| div().font_size(px(m3.type_.title_medium)).font_weight(FontWeight::SEMIBOLD)
            .text_color(m3.m3.on_surface).child(tr(s).to_string());
        let mut card = card(&m3).flex().flex_none().flex_col().gap(px(12.)).w(px(320.)).p(px(20.));
        if let Some(line) = summary {
            card = card.child(heading("What's new"))
                .child(div().font_size(px(m3.type_.title_large)).font_weight(FontWeight::BOLD)
                    .text_color(m3.m3.primary)
                    .child(horizon_launcher::nightly::label(tag)
                        .unwrap_or_else(|| tag.trim_start_matches('v').to_string())))
                .child(div().font_size(px(m3.type_.body_medium)).text_color(m3.m3.on_surface_variant)
                    .line_clamp(3).child(line))
                .child(div().relative().my(px(10.)).h(px(1.)).bg(m3.m3.outline_variant));
        }
        card.child(heading("Join a friend")).child(self.friend_box(self.can_start(tag), cx))
    }

    fn friend_box(&self, can_play: bool, cx: &mut Context<Self>) -> Div {
        let t = self.theme;
        let m3 = M3Theme::for_accent(self.settings.accent, t.dark);
        let focused = self.friend_focused;
        let field = div().id("friend-code").track_focus(&self.friend_focus).flex().flex_1().min_w_0()
            .items_center().h(px(theme::CONTROL)).px(px(12.)).overflow_hidden()
            .rounded(px(m3.shapes.medium)).border_1()
            .border_color(if focused { m3.m3.primary } else { m3.m3.outline_variant })
            .bg(m3.m3.surface_variant).font_family(theme::MONO).text_size(px(12.)).cursor_text()
            .child(line_edit::content(&self.friend_code, &t, focused, false, false,
                tr("Paste an invite code")))
            .on_click(cx.listener(|this, _: &ClickEvent, window, cx| {
                window.focus(&this.friend_focus, cx); cx.notify();
            }))
            .on_key_down(cx.listener(|this, ev: &KeyDownEvent, window, cx| {
                match line_edit::handle(&mut this.friend_code, ev, cx) {
                    line_edit::Key::Pass => return,
                    line_edit::Key::Enter => this.join_friend(),
                    line_edit::Key::Escape => window.blur(cx), _ => {}
                }
                cx.stop_propagation(); cx.notify();
            }));
        let field = line_edit::mouse(field, cx, |this| &mut this.friend_code);
        let valid = horizon_friends::invite::parse(self.friend_code.text()).is_some();
        let mut join = pill(&m3).child(label(tr("Join").to_string()));
        if can_play && valid {
            join = join.on_click(cx.listener(|this, _: &ClickEvent, _, cx| {
                this.join_friend(); cx.notify();
            }));
        } else { join = disabled(join); }
        div().relative().flex().items_center().gap(px(8.)).child(field).child(join)
    }
}

impl Launcher {
    pub(super) fn download_left(&mut self) -> Option<Duration> {
        if getting_ready_preview() { return Some(Duration::from_secs(70)); }
        let Some((done, total)) = horizon_launcher::mojang::progress() else {
            self.download_seen = None; return None;
        };
        let (since, from) = match self.download_seen {
            Some(seen) if seen.1 <= done => seen,
            _ => { self.download_seen = Some((Instant::now(), done)); return None; }
        };
        time_left(since.elapsed(), from, done, total)
    }

    pub(super) fn steps(&self, version: &str, left: Option<Duration>) -> Div {
        let t = self.theme;
        let n = t.night;
        let now = backend::prepare_step();
        let download = download_progress();
        let doing = match (download, now) {
            (Some((done, total)), backend::PrepareStep::Files) => trf(
                "Downloading Minecraft {version} from Mojang · {done} of {total} files",
                &[("version", version), ("done", &thousands(done as u64)), ("total", &thousands(total as u64))]),
            (Some((done, total)), _) => trf("Downloading from Mojang · {done} of {total} files",
                &[("done", &thousands(done as u64)), ("total", &thousands(total as u64))]),
            (None, _) => self.prepare_log.back().cloned().unwrap_or_default(),
        };
        let left = left.map(|d| match d.as_secs().div_ceil(60) {
            0 | 1 => tr("about a minute left").to_string(),
            m => trf("about {n} min left", &[("n", &m.to_string())]),
        });
        let play = self.play_when_prepared.is_some();
        let pill = |name: &str, state: std::cmp::Ordering| {
            let (mark, color) = match state {
                std::cmp::Ordering::Less => (Some(blocks::icon("check", 12., c(n.success))), c(n.ink_soft)),
                std::cmp::Ordering::Equal => (None, c(n.ink)),
                std::cmp::Ordering::Greater => (None, c(n.muted)),
            };
            div().flex().items_center().gap(px(6.))
                .child(match mark { Some(icon) => icon.into_any_element(),
                    None => div().size(px(6.)).bg(if state == std::cmp::Ordering::Equal { c(n.accent_hi) } else { alpha(n.muted, 0.5).into() }).into_any_element() })
                .child(shell::text(night::CAPTION, color).font_weight(FontWeight::MEDIUM).child(tr(name).to_string()))
        };
        div().flex().flex_col().gap(px(10.)).max_w(px(620.))
            .child(div().flex().child(progress_bar(download.map(|(done, total)| { let f = done as f32 / total as f32; (f, f) }), &t)))
            .child(div().flex().justify_between().gap(px(16.))
                .child(shell::text(night::CAPTION, c(n.ink_soft)).min_w_0().truncate().child(doing))
                .children(left.map(|l| shell::text(night::CAPTION, c(n.muted)).flex_none().child(l))))
            .child(div().flex().flex_wrap().gap(px(16.))
                .children(backend::PrepareStep::ALL.iter().map(|s| pill(s.name(), s.cmp(&now))))
                .children(play.then(|| pill("Play", std::cmp::Ordering::Greater))))
    }
}

pub(super) fn getting_ready_preview() -> bool {
    static PREVIEW: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *PREVIEW.get_or_init(|| std::env::var("HORIZON_LAUNCHER_PAGE").as_deref() == Ok("getting-ready"))
}

fn download_progress() -> Option<(usize, usize)> {
    horizon_launcher::mojang::progress().or_else(|| getting_ready_preview().then_some((1754, 5160)))
}

fn time_left(spent: Duration, from: usize, done: usize, total: usize) -> Option<Duration> {
    (spent >= Duration::from_secs(5) && done > from)
        .then(|| spent.mul_f64(total.saturating_sub(done) as f64 / (done - from) as f64))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn time_left_follows_the_pace_since_first_seen() {
        let s = Duration::from_secs;
        assert_eq!(time_left(s(10), 100, 600, 3100), Some(s(50)));
        assert_eq!(time_left(s(4), 100, 600, 3100), None, "too early");
        assert_eq!(time_left(s(10), 600, 600, 3100), None, "nothing done yet");
        assert_eq!(time_left(s(10), 0, 50, 50), Some(s(0)));
    }
    #[test]
    fn steps_run_in_their_order() {
        use backend::PrepareStep::*;
        assert_eq!(backend::PrepareStep::ALL, [Files, Java, Sounds, Prepare]);
        for s in backend::PrepareStep::ALL {
            assert_eq!(backend::PrepareStep::ALL[s as usize], s);
        }
    }
}

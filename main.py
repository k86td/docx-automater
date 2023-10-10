# from textual.app import App, ComposeResult
# from textual.containers import ScrollableContainer
# from textual.widgets import Header, Footer, Button, Static
# from textual.reactive import reactive
#
# from time import monotonic
#
# class TimeDisplay(Static):
#     """A Widget to display displayed time"""
#
#     start_time = reactive(monotonic)
#     time = reactive(0.0)
#     total = reactive(0.0)
#
#     def on_mount(self) -> None:
#         self.update_timer = self.set_interval(1 / 60, self.update_time, pause=True)
#
#     def update_time(self) -> None:
#         self.time = self.total + (monotonic() - self.start_time)
#
#     def watch_time(self, time: float) -> None:
#         minutes, seconds = divmod(time, 60)
#         hours, minutes = divmod(minutes, 60)
#         self.update(f"{hours:02,.0f}:{minutes:02.0f}:{seconds:05.2f}")
#
#     def start(self) -> None:
#         self.start_time = monotonic()
#         self.update_timer.resume()
#
#     def stop(self) -> None:
#         self.update_timer.pause()
#         self.total += monotonic() - self.start_time
#         self.time = self.total
#
#     def reset(self) -> None:
#         self.total = 0
#         self.time = 0
#
# class Stopwatch(Static):
#     """A stopwatch widget"""
#     def compose(self) -> ComposeResult:
#         yield Button("Start", id="start", variant="success")
#         yield Button("Stop", id="stop", variant="error")
#         yield Button("Reset", id="reset")
#         yield TimeDisplay()
#
#     def on_button_pressed(self, event: Button.Pressed) -> None:
#         button_id = event.button.id
#         time_display = self.query_one(TimeDisplay)
#         if button_id == "start":
#             time_display.start()
#             self.add_class("started")
#         elif button_id == "stop":
#             time_display.stop()
#             self.remove_class("started")
#         elif button_id == "reset":
#             time_display.reset()
#
# class StopwatchApp(App):
#     CSS_PATH = "stopwatch.tcss"
#     BINDINGS = [
#         ("d", "toggle_dark", "Toggle Dark Mode"),
#         ("a", "add_stopwatch", "Add a new stopwatch"),
#         ("x", "remove_stopwatch", "Remove the last stopwatch")
#     ]
#
#     def compose(self) -> ComposeResult:
#         yield Header()
#         yield Footer()
#         yield ScrollableContainer(id = "timers")
#
#     def action_toggle_dark(self) -> None:
#         self.dark = not self.dark
#
#     def action_add_stopwatch(self) -> None:
#         new_stopwatch = Stopwatch()
#         self.query_one("#timers").mount(new_stopwatch)
#         new_stopwatch.scroll_visible()
#
#     def action_remove_stopwatch(self) -> None:
#         timers = self.query(Stopwatch)
#         if timers:
#             timers.last().remove()
#
# if __name__ == "__main__":
#     app = StopwatchApp()
#     app.run()

import sys
from datetime import datetime
from textual.screen import Screen, ModalScreen
from textual.app import App, ComposeResult
from textual.widgets import Footer, Label, Markdown, TabbedContent, TabPane, Static, Input, Rule, Button
from textual.containers import Horizontal, Vertical

SIGNALEMENT_MENU = """
C'est ton menu de gestion de signalements! Tu n'as qu'à choisir si tu veux avoir une \<liste\>, ou en \<ajouter\>!
"""

class TabbedApp(App):
    """An example of tabbed content."""

    CSS_PATH = "style.tcss"
    BINDINGS = [
        ("escape", "exit", "Quitter"),
        # ("l", "show_tab('leto')", "Leto"),
        # ("j", "show_tab('jessica')", "Jessica"),
        # ("p", "show_tab('paul')", "Paul"),
    ]

    def action_show_tab(self, tab: str) -> None:
        """Switch to a new tab."""
        self.get_child_by_type(TabbedContent).active = tab

    def action_exit(self) -> None:
        self.app.exit()

    def compose(self) -> ComposeResult:
        """Compose app with tabbed content."""
        # Footer to show keys
        yield Footer()

        # Add the TabbedContent widget
        with TabbedContent(initial="signalements"):
            with TabPane("Signalements", id="signalements"):
                yield Markdown(SIGNALEMENT_MENU)
                with TabbedContent(initial="list_signalements", id="signalement_actions"):
                    with TabPane("Liste", id="list_signalements"):
                        yield Markdown("Voici une liste de tout tes signalements!")

                    with TabPane("Ajouter", id="add_signalements"):
                        yield Markdown("Ici, tu peux ajouter un nouveau signalement")
                        yield Input(placeholder="Nom")
                        yield Input(placeholder="Groupe")
                        yield Input(placeholder="Date", value=datetime.today().strftime('%Y-%m-%d'))
                        yield Input(placeholder="Éducateur")

                        yield Rule(orientation="horizontal", line_style="double")
                        yield Horizontal (
                            Button(label="Ajouter", variant="success", classes="action_buttons", id="but_add_signalement"),
                            Button(label="Réinitialiser", variant="error", classes="action_buttons", id="but_reset_signalement")
                        )

                        yield Static()


if __name__ == "__main__":
    app = TabbedApp()
    app.run()

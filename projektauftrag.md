# Shock Clock

## Beschreibung

Das Projekt kann in zwei wesentliche Teile aufgeteilt werden:

  1. tatsächliche "Shock Clock": Ein Elektroshocker
     (welcher aus einem kommerziellem Produkt stammt -> keine Verletzungsgefahr),
     der von einer Mobile-App aus ausgelöst werden kann
  2. Mobile-App: Mit dieser App lässt sich eine vom User erstellte Bedingung erstellen.
     Bei Erfüllung jener Bedingung wird ein Bluetooth Signal an die Uhr gesendet,
     welche dann einen Stromschlag abgibt.
     Bedingungen umfassen Aktionen wie das Öffnen einer anderen App.
Die Idee basiert auf dem Prinzip der Aversionstherapie, mit der man
schlechte Angewohnheiten durch negative Reize (hier Stromschläge) abgewöhnen kann.

## Geplantes Ergebnis

Android App, welche den Elektroshocker über Bluetooth kontrollieren kann.
Sollte die Zeit reichen, ist eine Desktopintegration für das
Windows-Betriebssystem geplant. Diese soll sich über einen zentralen Server mit
dem Handy des Nutzers verbinden können, um so den Stromschlag zu triggern.

## Team-Informationen

Name des Teams: Shock-Clock

Teamleiter: Vincent Winkler

Teammitglieder:

* Vincent Winkler (3BHIF)
* Jason Puschnig (3BHIF)

## Eingesetzte Technologien

* Rust (Tauri + Leptos + Tokio)
* Arduino C
* Kotlin
* Sqlite

/label ~state::submitted

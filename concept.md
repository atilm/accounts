# Accounts

## Bestandteile des Programms


* Parsen verschiedener Quelldateien
  * Mehrere Quelldateien für ein Konto einlesen
  * Verschiedene Quelldateien Erkennen
    * DKB Konto
    * DKB Kreditkarte
    * ING Konto

* Datenmodell für Konto
  * Art des Kontos
  * Aktueller Kontostand und Datum
  * Liste von Buchungen
    * Datum
    * Betrag
    * Auftraggeber
    * Buchungstext
    * Verwendungszweck

* Datenmodell Kategorien
  * Name
  * Erkennung pro Kontoart

* Algorithmen auf Daten
  * Kontostand rekonstruieren und plotten
  * Mehrere Kontenhistorien von verschiedenen Konten zu einer Gesamthistorie vereinigen


## Aufgaben

* Kontoauszuüge einlesen
  * Von diesen Quellen
    * DKB Konten
    * DKB Kreditkarten
    * ING Konten
  * Diese Informationen
    * Aktueller Kontostand
    * Wann
    * Wieviel
    * Einzahlung / Abbuchung
    * Von wem / an wen?

* Items Kategorisieren
  * Konfigurierbar (vlt. im Code): Gehalt, Essen, Wohnen, Kind, Sparen, ...

* Analyse
  * Verlauf der Kontostände rekonstruieren
  (* Verlauf des Gesamtvermögens rekonstruieren)
  * Überweisungen zwischen eigenen Konten
    * ignorieren ?
    * abgleichen ?
  * Monatliches Saldo erstellen:
    * Summe pro Kategorie
    * Summe Eingänge
    * Summe Ausgänge
    * Saldo

  
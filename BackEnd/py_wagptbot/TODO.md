# TODO

## Proof-of-concept connections

```mermaid
flowchart LR
    W("WhatsApp") ---|SMS| T("Twilio")
    T ---|HTTP| A["App"]
    A --- G("OpenAI GPT")
    A --- D[(SQL)]
    A --- L[/log/]
```

- [ ] (Twilio - WhatsApp) connection
- [ ] (App - Twilio) connection
- [ ] (App - GPT) connection
- [ ] (App - DB & log) workflow

## Busines logic
- [ ] Process 

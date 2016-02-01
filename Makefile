# 'Makefile'
MARKDOWN = pandoc --from markdown_github --to html --standalone
PUB_SRV_HOST = "ratina.org"
PUB_SRV_USER = "root"
PUB_SRV_PATH = "/var/www/pub/rust-notes"

HTML = "notes.html"

all: $(patsubst %.md,%.html,$(wildcard *.md)) Makefile

clean:
	rm -f $(patsubst %.md,%.html,$(wildcard *.md))
	rm -f *.bak *~

%.html: %.md
	$(MARKDOWN) $< --output $@

preview: all
	xdg-open $(HTML)

pub: all
	rsync -avz -e "ssh -o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null" --progress notes.html $(PUB_SRV_USER)@$(PUB_SRV_HOST):$(PUB_SRV_PATH)

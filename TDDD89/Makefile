all: edvth289_seminar_1.pdf outline.pdf thesis.pdf edvth289_seimnar_2.pdf

.PHONY: all thesis

%.pdf: %.md
	pandoc $< -tpdf > $@ 

%.pdf: % %.tex %.bib
	tectonic --keep-intermediates --reruns 0 $<.tex
	biber $<
	tectonic $<.tex --outfmt pdf

clean:
	rm *.aux
	rm *.bbl
	rm *.bcf
	rm *.pdf

thesis:
	cp sources.bib liuthesis/edvth289_master_thesis.bib
	cp sources.bib liuthesis/references.bib
	cp *.tex liuthesis/
	make -C liuthesis

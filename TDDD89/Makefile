all: edvth289_seminar_1.pdf outline.pdf thesis.pdf

.PHONY: all thesis

%.pdf: %.md
	pandoc $< -tpdf > $@ 

%.pdf: %.tex *.tex sources.bib
	$(shell cpp $< | sed "s/^\#/\%% PRE %%/" > temp.tex)
	cp sources.bib temp.bib
	tectonic --keep-intermediates --reruns 0 temp.tex
	biber temp
	tectonic temp.tex --outfmt pdf
	cp temp.pdf $@

clean:
	rm *.aux
	rm *.bbl
	rm *.bcf
	rm *.pdf

thesis:
	cp sources.bib liuthesis/edvth289_master_thesis.bib
	cp *.tex liuthesis/
	make -C liuthesis

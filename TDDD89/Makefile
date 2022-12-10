all: edvth289_seminar_1.pdf outline.pdf thesis.pdf edvth289_seimnar_2.pdf sem_4_ans.pdf

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

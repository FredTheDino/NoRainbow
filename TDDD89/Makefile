
all: edvth289_seminar_1.pdf outline.pdf edvth289_extended_thesis_plan_seminar_2.pdf

%.pdf: %.md
	pandoc $< -tpdf > $@ 

%.pdf: %.tex %.bib
	tectonic --keep-intermediates --reruns 0 $<
	biber $(shell basename -s .tex $<)
	tectonic $< --outfmt pdf

clean:
	rm *.aux
	rm *.bbl
	rm *.bcf
	rm *.pdf

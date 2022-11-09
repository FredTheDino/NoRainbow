**Read the chapters about Introduction, Background and Related work (sometimes called Theory) of the published master's thesis pertaining to your group, and glimpse through the rest of the thesis.**

# Are the research questions in the published thesis easy to find, clear, and with a reasonable scope, as required by the Instructions for final thesis reports?

They are easy to find since they have their own heading:

## Q.1
> How does automated priority assignment to tasks in an application modeled as
> a directed graph (DG) with soft real-time requirements affect system
> performance with regards to some soft real-time requirement compliance
> metric?

It's not clear what the scope is. "some soft real-time ... metrics" is not very actionable and clearly scoped.
This might however be reasonable to allow some flexibility in how much time is put on the thesis project.
Since more metrics easily can be added at the end.

## Q.2
> How can such an automated priority assignment strategy be designed?

Clear in what the desired result is, but unclear in what to describe. This is
possibly because a clearer question would itself answer this question. The question is very open ended, making the scope potentially to big.

## Q.3
> Can discrete-event simulation (DES) be used to accurately simulate and
> evaluate the effects that different priority assignments have on the
> application?

It would be clearer if they used a specific way of measuring.

## General points
The questions are easy to find and clear. The scope is quite flexible, which
might be a problem further down the line. To improve the questions further one
could limit the range of the research. Since this is an incremental improvement
it's probably well known how to measure (which is the main source of vague-ness).

# What are the promised (if any, usually to be found in the abstract and/or introduction) type of result and concrete contributions of the thesis, and how do these relate to the listed research questions?

The abstract claim that discrete event simulation can be used to evaluate task priority assignments, showing positive results in an actual system.

There are a few claims:
  - A real system is used as a base point
  - The simulations using DES correlated with how the system performed in some areas
  - Improvements were made
  - There is enough proof to show that DES can be used to evaluate task priority

# Are the research questions followed up and actually answered later in the thesis?

I skimmed the conclusion. The second research question was not mentioned there
but was answered in the text in section 8.2.2 - though could have been clearly
stated again in the conclusion to aid readability.

# How do you interpret the items in the grading rubric? Are any items difficult to understand?
I find it hard to understand what a `lucid development of the thesis` mean. It also doesn't feel application to the sections I've read thoroughly to express critical thinking of the ideas presented -- is it a good idea to describe bad ideas?

# How would you assess the chapters covering Introduction and Background + Related work ("Theory") of the thesis based on the grading rubric?
In general good. Well structured and easy to extract the information where it's needed. I personally disagree with the style of language, but that's a side note.

There are some sentences like
> The choice to implement the simulator in Python3 made the required
> development effort relatively small compared to some other languages such as
> C or C++, but did also result in a slower implementation. 

This sentence is long, complicated and vauge. What is a slow implementation?
Was it the implementation effort or the execution of the implementation that
was slow? What does the word `some` add in the sentence? But these again, are personal gripes.

 - Introduction: PASS. Fairly short and to the point. It definitely peeks my interest. I'd mark it as passing.
 - Background + Related Work (Theory): APPROVE. Maybe the related works sections should be longer. But what is present is of high quality. Some parts are less supported than others, the search algoritm part is well supported for example, but section 3.1 is not very well supported. 
 - Method (and Plan): APPROVE. I'm not sure there are much alternatives to the plan of implementing this discrete simulation and seeing what it does. 
 - Organization: PASS. The first chapters are well structured, but they don't leave much room for elaboration.
 - Language and form: PASS. I've not found spelling mistakes. The wording is good and it reads well. Sometimes the sentences are needlessly complex. But I'd give this a passing grade.

# What are the most common causes of plagiarism or copyright infringement do you think? How can you work to avoid these issues?

I think the most common way is to simply copy a piece of text and forget to
credit. The second most common I suspect is self plagiarism. These can be
counter acted by simply not copying text. Making sure you use your own words to
describe things and properly referencing sources where applicable is what I
suspect is the easiest way.

I personally add sources aggressively in to my bibtex files to lower the
barrier for correct referencing from the start.

The `related work` section was quite extensive though giving me confidence in the authors.

# Are there violations to the Guidelines on plagiarism in the thesis? For instance, is it clear that all figures are created by the author or used with expressed permission?
I've only skimmed the thesis. But the graphs contain their own data from what I have seen. I doubt that these figures are someone else's.

# What insights did you already have, or gain from the panel discussion of 1/11, for the selection of your future thesis project topic and for your personal career plans?
It sounded like there is a larger gap between academia and industry than I thought. Maybe I should put more time into thinking which track I actually want to take.
I also felt motivated to just try stuff -- that that is the best way to learn.

# And what kind of information is, from your point of view, still missing in order to outline a more informed plan towards your personal future professional activities and career steps? Where could you get such information? 
I don't feel like I lack any information. I personally feel like sitting in a
really comfy chair thinking about computer problems, preferably with some tea,
is what I want from my life. But I have to ask myself where I get to do this
the most. Or maybe this is the wrong goal for me altogether.

If I wanted more information, I would ask people I know who are taking their PhD.

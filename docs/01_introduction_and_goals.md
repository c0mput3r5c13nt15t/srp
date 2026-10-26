# Introduction and Goals

<div class="sidebar">

<div class="title">

</div>

Describes the relevant requirements and the driving forces that software
architects and development team must consider. These include

- underlying business goals,

- essential features,

- essential functional requirements,

- quality goals for the architecture and

- relevant stakeholders and their expectations

</div>

## Requirements Overview

<div class="sidebar">

<div class="title">

</div>

<div class="formalpara">

<div class="title">

Contents

</div>

Short description of the functional requirements, driving forces,
extract (or abstract) of requirements. Link to (hopefully existing)
requirements documents (with version number and information where to
find it).

</div>

<div class="formalpara">

<div class="title">

Motivation

</div>

From the point of view of the end users a system is created or modified
to improve support of a business activity and/or improve the quality.

</div>

<div class="formalpara">

<div class="title">

Form

</div>

Short textual description, probably in tabular use-case format. If
requirements documents exist this overview should refer to these
documents.

</div>

Keep these excerpts as short as possible. Balance readability of this
document with potential redundancy w.r.t to requirements documents.

<div class="formalpara">

<div class="title">

Further Information

</div>

See [Introduction and Goals](https://docs.arc42.org/section-1/) in the
arc42 documentation.

</div>

</div>

## Quality Goals

<div class="sidebar">

<div class="title">

</div>

<div class="formalpara">

<div class="title">

Contents

</div>

The top three (max five) quality goals for the architecture whose
fulfillment is of highest importance to the major stakeholders. We
really mean quality goals for the architecture. Don’t confuse them with
project goals. They are not necessarily identical.

</div>

Consider this overview of potential topics (based upon the [ISO
25010:2023](https://www.iso.org/obp/ui/#iso:std:iso-iec:25010:ed-2:v1:en)
standard):

<figure>
<img src="images/01_2_iso-25010-topics-EN-2023.drawio.png"
alt="Categories of Quality Requirements" />
</figure>

For a more extensive (and open) collection of qualities, definitions and
example requirements, see the [Q42 quality
model](https://quality.arc42.org).

<div class="formalpara">

<div class="title">

Motivation

</div>

You should know the quality goals of your most important stakeholders,
since they will influence fundamental architectural decisions. Make sure
to be very concrete about these qualities, avoid buzzwords. If you as an
architect do not know how the quality of your work will be judged…​

</div>

<div class="formalpara">

<div class="title">

Form

</div>

A table with quality goals and concrete scenarios, ordered by priorities

</div>

</div>

## Stakeholders

<div class="sidebar">

<div class="title">

</div>

<div class="formalpara">

<div class="title">

Contents

</div>

Explicit overview of stakeholders of the system, i.e. all person, roles
or organizations that

</div>

- should know the architecture

- have to be convinced of the architecture

- have to work with the architecture or with code

- need the documentation of the architecture for their work

- have to come up with decisions about the system or its development

<div class="formalpara">

<div class="title">

Motivation

</div>

You should know all parties involved in development of the system or
affected by the system. Otherwise, you may get nasty surprises later in
the development process. These stakeholders determine the extent and the
level of detail of your work and its results.

</div>

<div class="formalpara">

<div class="title">

Form

</div>

Table with role names, person names, and their expectations with respect
to the architecture documentation (!). Do not repeat stakeholders'
functional and quality requirements. Identify their documentation needs
here.

</div>

<div class="formalpara">

<div class="title">

Example

</div>

The following stakeholder roles recur across many projects. Use it as a
starting point, then adapt the roles, names and expectations to your
actual project. Not every project has all of these roles, and some will
have additional ones.

</div>

| Role/Name | Contact | Expectations |
|----|----|----|
| Product Owner | Product Management | Prioritizes features by business value; needs Section 1 (Requirements Overview, Quality Goals) kept aligned with product strategy |
| End Users / Customers | represented via UX research, support tickets | Need the system to be usable, reliable and fast; drive quality goals such as performance and usability |
| Software Architect(s) | Architecture Team | Own and maintain this documentation; use it to justify and communicate technical decisions |
| Development Team | Engineering Team | Need clear building blocks (Section 5) and runtime scenarios (Section 6) to implement correctly |
| Operations / SRE | Ops Team | Need the deployment view (Section 7) and cross-cutting concepts (Section 8, e.g. monitoring, logging) to run the system reliably |
| Security Officer / InfoSec | Security Team | Review the architecture against risks (Section 11) and quality requirements (Section 10); need evidence that security constraints are addressed |
| Compliance / Legal | Legal/Compliance Team | Need evidence that regulatory constraints (Section 2) are respected, e.g. data protection, licensing |
| Project Sponsor / Management | Steering Committee | Need confidence that architecture decisions (Section 9) support budget, timeline and strategic goals |

</div>

| Role/Name    | Contact         | Expectations        |
|--------------|-----------------|---------------------|
| *\<Role-1\>* | *\<Contact-1\>* | *\<Expectation-1\>* |
| *\<Role-2\>* | *\<Contact-2\>* | *\<Expectation-2\>* |

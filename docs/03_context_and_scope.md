# Context and Scope

<div class="sidebar">

<div class="title">

</div>

<div class="formalpara">

<div class="title">

Contents

</div>

Context and scope - as the name suggests - delimits your system (i.e.
your scope) from all its communication partners (neighboring systems and
users, i.e. the context of your system). It thereby specifies the
external interfaces.

</div>

If necessary, differentiate the business context (domain specific inputs
and outputs) from the technical context (channels, protocols, hardware).

<div class="formalpara">

<div class="title">

Motivation

</div>

The domain interfaces and technical interfaces to communication partners
are among your system’s most critical aspects. Make sure that you
completely understand them.

</div>

<div class="formalpara">

<div class="title">

Form

</div>

Various options:

</div>

- Context diagrams

- Lists of communication partners and their interfaces.

<div class="formalpara">

<div class="title">

Further Information

</div>

See [Context and Scope](https://docs.arc42.org/section-3/) in the arc42
documentation.

</div>

</div>

## Business Context

<div class="sidebar">

<div class="title">

</div>

<div class="formalpara">

<div class="title">

Contents

</div>

Specification of **all** communication partners (users, IT-systems, …​)
with explanations of domain specific inputs and outputs or interfaces.
Optionally you can add domain specific formats or communication
protocols.

</div>

<div class="formalpara">

<div class="title">

Motivation

</div>

All stakeholders should understand which data are exchanged with the
environment of the system.

</div>

<div class="formalpara">

<div class="title">

Form

</div>

All kinds of diagrams that show the system as a black box and specify
the domain interfaces to communication partners.

</div>

Alternatively (or additionally) you can use a table. The title of the
table is the name of your system, the three columns contain the name of
the communication partner, the inputs, and the outputs.

</div>

**\<Diagram or Table\>**

**\<optionally: Explanation of external domain interfaces\>**

## Technical Context

<div class="sidebar">

<div class="title">

</div>

<div class="formalpara">

<div class="title">

Contents

</div>

Technical interfaces (channels and transmission media) linking your
system to its environment. In addition a mapping of domain specific
input/output to the channels, i.e. an explanation which I/O uses which
channel.

</div>

<div class="formalpara">

<div class="title">

Motivation

</div>

Many stakeholders make architectural decision based on the technical
interfaces between the system and its context. Especially infrastructure
or hardware designers decide these technical interfaces.

</div>

<div class="formalpara">

<div class="title">

Form

</div>

E.g. UML deployment diagram describing channels to neighboring systems,
together with a mapping table showing the relationships between channels
and input/output.

</div>

</div>

**\<Diagram or Table\>**

**\<optionally: Explanation of technical interfaces\>**

**\<Mapping Input/Output to Channels\>**

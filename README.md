Protocol Manager
----------------

### About
So as the project stands now, I had an idea to make a protocol agnostic
messaging server. Currently however, I have not gotten much experience on
messaging protocols, so development will first focus on the
[LIME Protocol](http://limeprotocol.org/). Originally I was going to do
[XMPP](http://xmpp.org/rfcs/rfc3920.html), however Rust's current parsing
support for XML is... okay, and not quite up to par with JSON parsing, so that's
where I'll stay for now. I hope to work on the XML parsing at some point, just
not the focus now.


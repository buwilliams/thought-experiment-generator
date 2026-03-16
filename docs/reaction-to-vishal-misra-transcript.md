# Reaction to Vishal Misra — Transcript

Source: [Reaction to Vishal Misra](https://www.youtube.com/watch?v=iHINpU_Di58) by Brett Hall

---

## Introduction

**Brett Hall** *(0:00)*: Let's listen to an excerpt of an interview with Vishal. And I think it's great because there's been so much confusion on this and there's only been a few voices out there talking about what we might say is the black and white distinction between existing AI and AGI so far and most of them seem to be in the orbit of David Deutsch. Not all but many. And so it's good to hear this professional who works in the area also making the same noises. So let's listen to Vishal and I'll comment.

---

## Recursive Self-Improvement and the Inductive Closure

**Interviewer** *(0:35)*: Maybe just walk through very quickly how like the same model you can just very quickly show that a model can never recursively self-improve.

**Vishal Misra** *(0:45)*: So another phrase that we've been using recently is you know the output of the LLM is the inductive closure of what it has been trained on.

**Interviewer**: Yeah.

**Vishal** *(1:00)*: So when you say that it can recursively self-improve, it could mean one of two things. So, let's get back to the—

**Interviewer** *(1:10)*: Well, actually, you know what's kind of interesting is like often the most people agree that if you have one LLM and you just feed the output into the input, like it's not going to do anything. But then often people will say, well, what if you have two LLMs, you have no external information, but you have two LLMs talking to each other. Maybe they can improve each other and then you can have like, you know, a takeoff scenario. But again, you even addressed this even in the case of like n number of LLMs using kind of the matrix model to show that like you just aren't gaining any information entropy.

**Vishal** *(1:42)*: Yeah. So, so you can represent the sort of information contained in these models and let's go back to that matrix analogy that I have — the matrix abstraction. So like I said you know these models represent a subset of the rows right?

**Interviewer**: Yeah.

**Vishal** *(2:04)*: So a subset of the rows are represented but some of these rows are able to help fill out some of the missing rows. For instance, you know, if the model knows how to do multiplication, doing the step by step, then every row that is corresponding to let's say 769 × 125 or whatever — all those — it can fill out the answer because it has those algorithms sort of embedded in them. You just need to unroll them.

**Interviewer**: Yeah.

**Vishal** *(2:33)*: So it can sort of self-improve up to a point. But beyond a point these models can only sort of generate what they have been trained on. So let me give you — I'll give you three examples.

---

## Brett's Commentary: No Escape from Training Data

**Brett Hall** *(2:51)*: So that's really important — this idea that you cannot have this recursive self-improvement from these models. This is something that many commentators have for want of another word been worried about. They're worried about the getaway self-improvement of maybe not LLMs but just AI more broadly. But if the thing isn't coming up with new explanations, it cannot just continue to improve itself without limit. And yet people do still argue that these LLMs are on the way to doing that. But as Vishal is saying, as I've argued as well, they can't get outside of their training data. They can only improve on new ways to reassemble the information that is there in the training library. They're not going to go beyond it and there is no way for them to go beyond it. Let's continue to listen.

---

## Vishal's Three Examples

**Vishal** *(3:51)*: So any model, any LLM that was trained on pre-1915 physics would never have come up with a theory of relativity. Einstein had to sort of reject the Newtonian physics and come up with this space-time continuum. He completely rewrote the rules, right? So that is an example of, you know, AGI.

**Brett** *(4:14)*: Again, I don't know if he's gotten to this independently, most likely, I suppose, but he may have listened to Deutsch. I don't know. But this is the whole point. You can't get to — or you can't even get to Newtonian physics from non-Newtonian physics. It is a creation. It's not a derivation. And given Newtonian physics, you're not going to get to Einstein. Einstein didn't look at Newtonian physics and extrapolate. He fixed problems with it, things that were absent from it that it could not possibly explain — that I've talked about on ToKCast many times before. And in doing so, as Vishal just said there, he rejected Newtonian. He rejected what had gone before in order to literally create something new. And he says that Einstein is the AGI, right? Well, he's a GI, but basically he's conceding that AGI would be people — where you are generating new knowledge. It's not simply computing. It's actually discovering something fundamental about the universe.

**Brett** *(5:27)*: Fundamental. And for that you have to go outside your training set. Similarly, you know, for that you have to go outside your training set. Yes. How? What does that mean though? Well, as I say, it's creativity. It is the thing that we don't understand that would need the algorithm for AGI presumably.

**Vishal** *(5:45)*: Any LLM that was trained on it would not have come up with quantum mechanics, right? That's wave-particle duality or this whole probabilistic notion or that you know energy is not continuous but it is quantized. You had to reject Newtonian physics.

**Interviewer**: Yeah.

**Vishal** *(6:02)*: Or get incompleteness theorem.

**Interviewer**: Yeah.

**Vishal** *(6:05)*: He had to go outside the axioms to say that okay it is incomplete. So those are examples where you're creating new science or fundamentally new results. That kind of self-improvement is not possible with these architectures.

---

## Brett's Commentary: Connecting Dots vs. Creating Dots

**Brett** *(6:22)*: Underscore that this kind of self-improvement is not possible with these LLMs. These LLMs can get better but not at that kind of thing. Not at inventing new explanations or new science or new anything really.

**Vishal** *(6:37)*: They can refine these — they can fill out these rows where the answer already exists. Another example, you know, which has received a lot of press these days is these IMO results, International Math Olympiad.

**Interviewer**: Yeah.

**Vishal** *(6:50)*: You know, whether it's a human solving it or the LLM solving it, they are not inventing new kinds of math.

**Interviewer**: Yeah.

**Vishal** *(7:00)*: They are able to connect known results in a sequence of steps to come up with the answer.

**Brett** *(7:09)*: This is what Paul Robachard, the mathematician that I interviewed as well with Conjecture Institute — go and have a look at that — what we were talking about: they can be very, very good these LLMs and various other kinds of AI, very good at solving highly mathematical puzzles like those found in maths olympiad examinations or tests or competitions. And some people who don't know maths are very, very impressed by their ability to do this. Even if you're not a mathematician. Even if you are a mathematician, I suppose you can be very impressed by that in the same way that you can sort of be impressed by a pocket calculator that can multiply two 12-digit numbers very, very quickly. Yeah. Okay. So, our electronics are fascinating and amazing.

Now, some people are just particularly impressed by a computer, a large language model that can do these complex math olympiad type questions or physics olympiad type questions, but they're not as impressed by the pocket calculator doing arithmetic. And yet, it's the same kind of thing. It's working with existing axioms in order to generate a result already known or at least already able to use processes, procedures, methodologies that have been discovered. They're not inventing new mathematics. That is using the existing mathematics as a set of axioms and then working through the rules of inference as a computer does. It's not creativity.

---

## Bayesian Manifolds and Entropy Collapse

**Vishal** *(8:38)*: So even the LLMs — what they're doing is they are exploring all sorts of solutions. In some of these solutions they start going on this path where their next token entropy is low. So that's where I say they are in that Bayesian manifold, where you have this entropy collapse and by doing those steps you arrive at the answer.

**Brett** *(9:05)*: By entropy collapse, he's using the word entropy as almost synonymous with uncertainty. And so entropy collapse means reducing the uncertainty down so that you've got a single response, the most likely response.

**Vishal** *(9:18)*: But you're not inventing new math. You're not inventing new axioms or new branches of mathematics. You're sort of using what you've been trained on to arrive at that answer.

**Interviewer**: Yeah.

**Vishal** *(9:29)*: So those things LLMs can do. You know, they'll get better at it — of connecting the known dots.

**Interviewer**: Yeah.

**Vishal** *(9:37)*: But creating new dots, I think we need an architectural advance.

**Brett** *(9:44)*: That's a lovely way of putting it. I might steal that from now on. LLMs and existing AI are very good at connecting known dots. They're not any good at creating new dots — the new discovery, the thing which will solve the problem that we have in gaps of our explanations, existing explanations. So in order to go further to get to that next new explanation that goes beyond just connecting the dots, we have to invent new dots and LLMs cannot do that. They are forever trapped by their training data.

---

## Defining AGI

**Interviewer** *(10:22)*: Yeah. So Martin was talking earlier about how the discourse was either stochastic parrots or you know AGI recursive self-improvement discourse — or even this the concept — what does it mean? To the extent that it's useful, how do you think about that?

**Vishal** *(10:42)*: So the way I think about it — the way we have tried to formulate in our papers — is it's beyond a stochastic parrot but it's not AGI. It's doing Bayesian reasoning over what it has been trained on. So it's a lot more sophisticated than just a stochastic parrot.

**Interviewer** *(10:59)*: Sorry, how do you define AGI?

**Vishal** *(11:02)*: Okay, so AGI — so how do I define AGI? So the way I would say that LLMs currently navigate through this known Bayesian manifold. AGI will create new manifolds.

**Brett** *(11:22)*: Asking for definitions is always a bad question. I think you know what would I say to this? Well, just what David Deutsch says, which is AGI is a person. So by general intelligence, we mean a person. So, an artificial person. If you want to get into the weeds, you say artificial universal explainer. What's a universal explainer? Well, then that opens up an entire discussion about what universality is and what explanations are and how they are created and the mystery of what it is to be a person. And it's a whole conversation — which is a black and white difference again from normal AI which are trapped within their training data. But no person and therefore no AGI would be trapped by training data. You go well beyond — you use your existing learning in order to invent, to create, to imagine brand new things that no one has ever thought of before.

But the LLM is only ever using the stuff that we've already discovered to just rearrange that stuff we've already discovered into perhaps novelty, but nothing creative, nothing that's not already in there — in the same way that any random shuffling of the 52 deck cards is there in the deck of cards, if you like. The 52 cards are the normal 52 cards. What's not in there is a joker and so what's not in there are the tarot cards and what's not in there is everything else that isn't a typical playing card.

---

## Navigation vs. Creation

**Vishal** *(13:02)*: So right now these models navigate — they do not create. AGI will be when we are able to create new science, new results, new math. When an AGI comes up with a theory of relativity — I mean it's an extremely high bar — but you get what I'm saying. It has to go beyond what it has been trained on to come up with new paradigms, new science. And that's my definition of AGI.

**Interviewer** *(13:30)*: Michelle, can you — do you think that based on the work you've done, can you bound the amount of data, compute, or data or compute that would be needed in order for it to evolve itself? So, one of the problems — if you just take LLMs as they exist is there was so much data used to create them. To create a new manifold, we'll need a lot more data just because of the basic mechanisms, right? Otherwise it'll just kind of get consumed into the existing set of data. Like, have you found any bounds of what would be needed to actually evolve the manifold in a useful way, or do you think we just need a new architecture?

**Vishal** *(14:12)*: I personally think that we need a new architecture. The more data that we have, the more compute we have, we'll get maybe smoother manifolds. So it's like a map.

---

## The Empiricism Objection

**Interviewer** *(14:22)*: Yeah. Because I mean there's this view that people have. They're like, "Well, Vish, this is all good and well, but you know, I could just take an LLM and I can give it eyes and I can give it ears and I can put it in the world and it'll gain information and based on that information, it'll improve itself." And therefore, it can learn new things. But—

**Vishal** *(14:44)*: That's the era of empiricism. Of course, empiricism means you can just go out into the world. You've got these eyes and you know what you're looking at and you can interpret what you're looking at accurately. If you gave an LLM — it's an experiment that people could do — if you gave an LLM cameras for eyes and you know, microphones for ears, the problem is — and even Jordan Peterson has talked about this — you have to know what to listen for. You have to know what to look at. Now the LLM no doubt and AI have already been trained. You know, the Roomba vacuum cleaner can be trained to look for particular things that the coder is already aware of. But if you give eyes and ears to this LLM thing in order that it constructs new knowledge, what does it look for?

Now if it's a person, no one has to tell it. It just becomes curious. A person is curious and looks around its environment and goes, "Hm, what's that? Never seen that before. Let me investigate." Especially if you're a scientist, you know, "I don't understand this thing. Let me try and come up with it." But at least I know, okay, I'm going to point the telescope at a star and look at a star. I know what a star is. The LLM won't know what it's looking for, what it's looking at. So this empiricist — it's pure empiricism to say you can just look and then understand the world. That's not the way this works. The data that comes into the cameras has to be interpreted given a theory. You have the theories first, then the data is collected and in light of those theories is interpreted. And it's when you don't understand what you're looking at that then you have to create in your mind the explanation of that thing that's out there.

**Brett** *(16:45)*: No doubt you can program an LLM. You know, mobile phones can already do this. You can point it at stuff and it will say okay that's a fan, that's a can of drink, that's a tree, that's a cat. The phone can actually do that. But for things it doesn't yet know — no clue. It doesn't begin to conjecture on things it doesn't appear in its library. And that's Vishal's point here. If it's not in the library, you can give it cameras and microphones all day long, high-fidelity cameras, very, very sensitive microphones. And if it hears something that's not there in its data, or it sees something that's not there in its data, it will just ignore it. It won't know what to do. What are you going to do? You're going to ask it, "What is that thing?" It'll give you an error or something.

You know, you point it at — let's say you pointed it at an insect that no one's ever seen before. Now, for a scientist, they can investigate and they can actually create explanations of what this insect is. Is it a bug or is it a fly, you know, etc. And it can develop theories. The LLM won't do such a thing because if it's not in the data then it won't.

Or any of a number of other mysteries — better would be actually explanatory mysteries. You know, because we can already do this kind of thing. So for example, and I know Sabine Hossenfelder has been one who's experimented with LLMs — and anyone can do this as well — but she's posted videos about this where you ask the LLM to invent new theories for dark matter. Let's say an existing problem that exists in astronomy, physics, science. So, we can already do that. We can already give the LLM these problems that we don't yet know the answers to and they're hopeless. They come up with laughable suggestions or just subtle variations on things they've already been programmed with. So, yeah, the cameras and the microphones won't help.

---

## New Architecture Needed

**Interviewer** *(18:49)*: The counterpoint that I've always just intuitively thought to that is the amount of data used to train these things is so large. How much can you actually evolve that manifold given an incremental — I mean almost none at all. Right? There has to be some other way to generate new manifolds that aren't evolving the existing one.

**Vishal** *(19:08)*: I completely agree. There has to be a new sort of architectural leap that is needed to go from the current, you know, just throwing more data and more compute — you know, it's going to plateau. It's, you know, the iPhone 15, 16, 17.

**Interviewer** *(19:23)*: And are there any research directions that are promising in your mind that might help us, you know?

---

## Brett's Closing Commentary

**Brett** *(19:28)*: Okay. So, I'll stop it there because he's going to talk about what he thinks are possible ways to AGI and he doesn't give anything that's new. He talks about Yann LeCun who's a famous researcher. I would say talk to Carlos De Agardia obviously because we know to some extent how knowledge is created. Then if you want AGI you have to have an understanding of Popperian epistemology. Now he doesn't mention that so whatever, but these guys — again even the interviewers here you can see where their bias for want of another word lays. They think like so many other researchers that you just throw more compute, you throw more data at these things and eventually they reach escape velocity and become AGI and then it keeps on going to ASI.

But as Vishal just said there, it's kind of like people complain about the iPhone. You know, when there wasn't iPhones and the iPhone appeared, it was a breakthrough, groundbreaking. And then for a little while the improvements in the iPhone were very great very quickly every new iPhone. But then, as he says, the 14, 15, 16, whatever 17 — the recent iPhones are very similar one to another. You know, I didn't bother getting the last two iPhones because I thought the one that I have, which I'm pretty sure it's the 15, is good enough and the improvements offered on the newer ones are just like slightly better camera, slightly longer battery life, that kind of thing. Now the same seems to be happening with the LLMs. They're fantastic. AI is fantastic, but the rate of improvement has slowed because you can only improve the amount of data or the quality of the data if you like, but you're not going to get much better. The responses are already as good as you would like — putting aside hallucinations and the bias that is sometimes injected into these things — but they do appear to be plateauing now, plateauing out at a fantastic level. I mean, what more do you want from these excellent chatbots and search engines? I think they're doing fantastic stuff.

But if what your aim is AGI, they talk about new architectures. Yes. New different philosophy, not just assuming that we can extrapolate from something that's not a person and is just a chatbot or a language calculator to something that is a creative explanation generator — things I've talked about ad nauseam on this channel over the last few months, indeed years. Now, a lot of people are now saying this, but a lot of people have been saying what Vishal has just said, that scaling LLMs won't get us to AGI. And yeah, David Deutsch has been saying that since the beginning. And I'm not with, by the way, this new group of wet blankets who think this is a huge failing of LLMs, the fact that they won't become AGI. It's not a huge failing. LLMs are excellent. Complaining they fail because they're not able to be scaled to AGI is rather like complaining that super fast rail will never actually fly. They're categorically different things and can be appreciated for the different things they do.

By the way, it's interesting too. Neither Grok nor ChatGPT nor Claude seem to know how they themselves work when I ask them. You know, you ask them to explain how LLMs work at a high level. By high level again I mean emergent not reductive. The interesting thing is when I interrogate ChatGPT about how is it able to achieve what it achieves — at least it says it has no beliefs and no LLMs have a belief.

Okay, that's enough for today. I promise that next time I will go back to regular programming unless some sort of world war breaks out or something like that. We will go back to The Fabric of Reality. But until then, goodbye.

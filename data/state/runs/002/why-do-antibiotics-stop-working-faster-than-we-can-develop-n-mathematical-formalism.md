# Generated: why-do-antibiotics-stop-working-faster-than-we-can-develop-n × mathematical-formalism

## Conjecture

Conjecture: the problem is governed by an asymmetric dynamical system in which bacterial adaptation has a much shorter characteristic timescale and a much larger accessible state space than antibiotic discovery, so resistance will systematically outpace drug development unless we change the system’s invariants rather than merely increase output.

Mathematically, let:

- \(B(t)\) = distribution of bacterial genotypes/phenotypes
- \(A(t)\) = stock of effective antibiotics
- \(R(t)\) = stock of resistance mechanisms available in the microbial world
- \(u(t)\) = antibiotic use intensity
- \(D(t)\) = discovery/development rate of new drugs

Antibiotic use is a selection operator \(S_u\) acting on \(B\), pushing population mass toward resistant states. Mutation, horizontal gene transfer, and huge population sizes define a transformation \(T\) that continually explores nearby genotype space. So resistance growth is roughly:

\[
\frac{dR}{dt} \approx f(N,\mu,H,u)
\]

where \(N\) is microbial population size, \(\mu\) mutation rate, and \(H\) horizontal transfer rate. All of these are large or effectively large. By contrast, effective antibiotic supply changes like:

\[
\frac{dA}{dt} = D(t) - L(t)
\]

where \(L(t)\) is loss of efficacy due to resistance. The key inequality is structural:

\[
L(t) > D(t)
\]

not accidentally, but because \(L\) scales with global bacterial reproduction and selection, while \(D\) scales with human R&D institutions, regulation, and economics.

The invariant doing the work is this: every use of a broadly deployed antibiotic tends to reduce its future effectiveness. So antibiotics are unlike ordinary technologies; consumption depletes informational value. Their “knowledge content” is perishable under selection pressure.

This reframes the issue. The bottleneck is not mainly chemical creativity, but search geometry. Bacteria search adaptively in parallel across immense populations, with immediate fitness feedback. We search serially through a narrow pipeline constrained by trials, markets, and safety requirements. Their search is coupled directly to reality; ours is buffered by institutions.

What follows is that “make more antibiotics faster” is a weak intervention unless it alters the mapping from use to resistance. High-leverage variables are those that change the system’s structure:

- reduce selection pressure \(u(t)\): stewardship, diagnostics, infection prevention
- reduce accessible resistance transitions: combination therapies, anti-evolution drugs, narrow-spectrum targeting
- reduce transmission and gene flow: sanitation, surveillance, containment
- increase persistence of efficacy rather than raw drug count

So the mathematical illumination is: this is a race between two search processes with unequal scaling laws. Bacteria win because their exploration of solution space is massively parallel, continuous, and rewarded by selection. We lose because we treat the problem as replenishing a stock, when the real task is redesigning the feedback loop that destroys that stock.

## Questions


## Candidate Problems

- Model the inequality L(t) > D(t) across pathogens, drug classes, and health systems to determine when it is structurally inevitable and when it can be reversed. (score: 0.93)
- Test which interventions most effectively change the mapping from antibiotic use u(t) to resistance growth dR/dt: stewardship, rapid diagnostics, narrow-spectrum drugs, combinations, anti-evolution adjuncts, or transmission control. (score: 0.90)
- Quantify how much horizontal gene transfer versus de novo mutation contributes to the accessible resistance state space for different bacterial ecologies. (score: 0.88)
- Determine whether some antibiotic designs can decouple consumption from depletion of future efficacy, weakening the claimed invariant that use erodes informational value. (score: 0.86)

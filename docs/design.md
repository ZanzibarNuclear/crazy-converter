# Crazy Converterator: Design Document

## What This Is

The Crazy Converterator helps people understand unfamiliar quantities by finding surprising, memorable comparisons between real-world items. It answers questions like: "How does the energy in a banana compare to nuclear decay?" or "What does a month of cow methane weigh in terms of something I can picture?"

People have good intuition for familiar things -- the heft of a banana, the length of a car, how long a lunch break is. They have terrible intuition for unfamiliar things -- the mass of an electron, the energy of a supernova, the age of the universe. By bridging the two with concrete, often absurd comparisons, we make the unfamiliar tangible and memorable.

The comparisons are a little bit silly on purpose. "A banana's caloric energy equals the radioactive decay of 47 trillion potassium-40 atoms" sticks in your head precisely because it's unexpected.

## Core Concepts

### Items

An **item** is a specific, concrete thing in the world: a banana, a cow, a human tooth, the Eiffel Tower, the Sun, an electron. Items are the nouns of the system. Each item has multiple measurable **properties**.

### Properties

A **property** is a measured quantity of an item in a specific dimension. A banana has:
- Energy: 89 kcal (caloric energy)
- Mass: 120 g (weight)
- Length: 20 cm (length)

A cow has:
- Mass: 700 kg (body weight)
- Mass: 6 kg/month (methane from flatulence)
- Volume: 40 liters/day (milk production)

Properties are the bridge between the familiar and the math engine. They turn "a banana" into "89 kcal" which the converter can work with.

### Dimensions

A **dimension** is a type of physical quantity: energy, mass, length, volume, time, temperature, etc. The Rust converter supports 14 dimensions. A comparison always operates within a dimension -- you can compare the energy of a banana to the energy of TNT, but not the energy of a banana to the length of a car (at least not in a single step).

### Comparisons

A **comparison** is the core output: given an item's property, find another item whose same-dimension property creates a surprising, memorable equivalence.

Input: "a banana" + "energy"
Output: "The caloric energy of 1 banana (89 kcal) equals the kinetic energy of a 2-ton car traveling at 15 mph."

Good comparisons have these qualities:
- **Cross-domain**: food vs. physics, animals vs. engineering, human body vs. astronomy
- **Surprising ratio**: either absurdly large (10 billion bananas) or absurdly small (0.00001 supernovas)
- **Tangible anchor**: at least one side should be something people can picture
- **Memorable**: slightly absurd, fun to share

### Chains

A **chain** is a multi-step comparison that hops across dimensions:

1. Start: "a cow's monthly methane output" → 6 kg of methane (mass)
2. Step 1: 6 kg of methane has the same energy as 2.3 gallons of gasoline (mass → energy)
3. Step 2: 2.3 gallons of gasoline could power a car for 75 miles (energy → distance)

Result: "One month of cow farts could drive you from New York to Philadelphia."

Chains are powerful because they connect things that seem completely unrelated, revealing non-obvious relationships. The dimension-hopping is what makes it "crazy."

## Architecture

### Layer 1: Conversion Math (Rust)

The existing Rust core. Pure unit-to-unit conversion within a single dimension. 14 dimension modules (time, length, area, volume, mass, speed, acceleration, force, pressure, energy, power, momentum, torque, temperature). Two-step pattern: source unit → base unit → target unit. Exposed to Python via PyO3.

**This layer doesn't change.** It's correct, fast, and does exactly what's needed. It's the "truth engine" -- when the system says "89 kcal = 372,372 joules," this layer guarantees the math is right.

### Layer 2: Item Knowledge (Python, data)

A structured catalog of real-world items with verified properties. Stored as JSON, loaded at startup. Each property ties to a Rust dimension via its `dimension` field.

The catalog is curated, not generated. Numbers are verified. This matters because the whole point is to build intuition -- wrong numbers would build wrong intuition.

The LLM can suggest items not in the catalog, but these are flagged as unverified. Over time, good LLM suggestions can be promoted to the catalog after verification.

The catalog should span many orders of magnitude within each dimension: subatomic to astronomical for mass, femtoseconds to billions of years for time, nanometers to light-years for length. This maximizes the surprise factor in comparisons.

### Layer 3: Comparison Engine (Python + LLM)

The creative brain. Given an item and dimension, it:
1. Looks up the item's property in that dimension
2. Asks the LLM to suggest a surprising comparison target
3. Uses the Rust converter to compute the exact equivalence
4. Generates a narrative explanation

The LLM's role is creative, not computational. It decides WHAT to compare (picking the most surprising/memorable target). The Rust converter decides the MATH (computing the exact equivalence). This separation keeps comparisons both creative and accurate.

For chains, the engine orchestrates multiple steps, with the LLM choosing each hop's target and dimension.

### Layer 4: UI (Nuxt)

A guided exploration interface, not just a chat box. The primary experience is a three-step flow:

1. **Pick an item** -- searchable grid with categories
2. **Pick a dimension** -- cards showing the item's available properties
3. **See the comparison** -- the result with narrative, math, and a visual sense of scale

A chat interface remains available as a secondary, freeform mode.

The UI's job is to teach users the system's model. Most people won't think to ask "compare a banana's energy to nuclear decay" unprompted. The UI should surface possibilities and guide them toward interesting comparisons.

## UX Principles

1. **Start with the familiar.** The entry point is always something the user knows: a banana, a car, their own body weight. Never start with the unfamiliar.

2. **Guide, don't gatekeep.** Help users find interesting comparisons, but don't prevent them from trying weird ones. The weird ones are often the best.

3. **Show the math, but lead with the story.** "One month of cow farts could drive you from New York to Philadelphia" comes first. The conversion chain (6 kg methane → energy → gasoline → miles) is available for the curious.

4. **Make it shareable.** The output should be a self-contained, memorable statement that works without context. People should want to share these.

5. **Stay within the constraints gracefully.** When the system can't do something (no property data, unsupported dimension), suggest alternatives rather than showing errors.

## Item Catalog Design

### Categories

- **Food**: banana, egg, pizza slice, chocolate bar, apple, cup of coffee
- **Animals**: cow, elephant, blue whale, ant, hummingbird, chicken
- **Human body**: human tooth, human hair, adult human, human brain, drop of blood
- **Vehicles**: car, bicycle, Boeing 747, Saturn V rocket, container ship
- **Buildings**: Eiffel Tower, Empire State Building, average house, Great Pyramid
- **Astronomy**: the Sun, the Moon, Earth, Jupiter, a neutron star, the Milky Way
- **Subatomic**: an electron, a proton, a hydrogen atom, a grain of sand
- **Natural phenomena**: lightning bolt, hurricane, earthquake (Richter 7), a tsunami
- **Time references**: a heartbeat, a blink, a school year, age of the universe
- **Energy references**: a AA battery, a stick of dynamite, a gallon of gasoline, Hiroshima bomb

### Property Selection

For each item, include properties in as many dimensions as are meaningful and non-obvious. Prioritize:
- Properties people don't usually think about (energy in food isn't just calories -- it has mass, volume, chemical energy in joules)
- Properties that create surprising cross-domain comparisons
- Properties where the numbers span extreme ranges

## What's Not In Scope (Yet)

- User accounts / persistence
- Custom item submission by users
- Real-time data (stock prices, weather)
- Non-physical dimensions (cost, popularity, beauty)
- Animated visualizations
- Mobile app

These are all possible Phase 3+ features but aren't needed to validate the core idea.

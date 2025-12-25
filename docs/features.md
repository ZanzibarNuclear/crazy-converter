# Crazy Converterator Features

## Overview

Crazy Converterator is a conversational unit conversion tool that uses natural language processing to understand user queries and perform accurate conversions across 14 different measurement categories.

## Core Features

### 1. Natural Language Conversion Queries

Users can ask conversion questions in natural language, such as:
- "Convert 5 miles to kilometers"
- "How many pounds is 10 kilograms?"
- "What's 100 degrees Fahrenheit in Celsius?"
- "Convert 2 hours to minutes"

The system uses an LLM (OpenAI or Anthropic) to understand the intent and extract the necessary conversion parameters.

### 2. Conversational Interface

- **Chat-based UI**: Clean, modern chat interface built with Vue.js
- **Conversation History**: Maintains context across multiple messages
- **Real-time Responses**: Streaming-style interface with loading indicators
- **Error Handling**: User-friendly error messages for connection and conversion issues

### 3. Comprehensive Unit Support

The system supports conversions across 14 categories:

#### Time
- Units: seconds, minutes, hours, days, weeks, months, years
- Abbreviations: s, sec, min, h, hr, hrs, d, w, mo, y, yr, yrs
- Base unit: seconds

#### Length
- Units: meters, centimeters, millimeters, micrometers, nanometers, kilometers, inches, feet, yards, miles
- Abbreviations: m, cm, mm, μm, um, nm, km, in, ", ft, ', yd, mi
- Base unit: meters

#### Area
- Units: square meters, square centimeters, square kilometers, square inches, square feet, square yards, acres, hectares
- Abbreviations: m², m2, cm², cm2, km², km2, in², in2, ft², ft2, yd², yd2
- Base unit: square meters

#### Volume
- Units: liters, milliliters, cubic meters, cubic centimeters, cubic inches, cubic feet, gallons (US/UK), fluid ounces, cups, pints, quarts
- Abbreviations: L, l, mL, ml, m³, m3, cm³, cm3, cc, in³, in3, ft³, ft3, gal, fl oz, floz
- Base unit: cubic meters

#### Mass
- Units: kilograms, grams, milligrams, pounds, ounces, tons (metric/US), stones
- Abbreviations: kg, g, mg, lb, lbs, oz, t
- Base unit: kilograms

#### Speed
- Units: meters per second, kilometers per hour, miles per hour, feet per second, knots
- Abbreviations: m/s, km/h, kmh, mph, ft/s, fps
- Base unit: meters per second

#### Acceleration
- Units: meters per second squared, feet per second squared, standard gravity
- Abbreviations: m/s², m/s2, ft/s², ft/s2, g
- Base unit: meters per second squared

#### Force
- Units: newtons, pounds-force, dynes, kilonewtons
- Abbreviations: N, lbf, dyn, kN
- Base unit: newtons

#### Pressure
- Units: pascals, kilopascals, megapascals, psi, bar, atmospheres, torr, millimeters of mercury
- Abbreviations: Pa, kPa, MPa, psi, bar, atm, torr, mmHg
- Base unit: pascals

#### Energy
- Units: joules, kilojoules, megajoules, kilowatt-hours, calories, kilocalories, BTU, foot-pounds, electronvolts
- Abbreviations: J, kJ, MJ, kWh, kw·h, cal, kcal, ev, kev, mev
- Base unit: joules

#### Power
- Units: watts, kilowatts, megawatts, horsepower, foot-pounds per second
- Abbreviations: W, kW, MW, hp, ft·lb/s
- Base unit: watts

#### Momentum
- Units: kilogram meters per second, pound feet per second, gram centimeters per second
- Abbreviations: kg·m/s, lb·ft/s, g·cm/s
- Base unit: kilogram meters per second

#### Torque
- Units: newton meters, pound-feet, kilogram-force meters, ounce-inches
- Abbreviations: N·m, lb·ft, kgf·m, oz·in
- Base unit: newton meters

#### Temperature
- Units: Celsius, Fahrenheit, Kelvin, Rankine
- Abbreviations: °C, C, celsius, °F, F, fahrenheit, K, kelvin, °R, R, rankine
- Base unit: Kelvin (for internal calculations)

### 4. High-Performance Conversions

- **Rust Implementation**: All conversion calculations performed in Rust for maximum performance
- **Python Integration**: Seamlessly integrated via PyO3 bindings
- **Type Safety**: Strong typing ensures correct unit handling
- **Precision**: High-precision floating-point calculations

### 5. LLM Integration

- **Multiple Providers**: Supports both OpenAI and Anthropic models
- **Configurable Models**: Choose from various models (e.g., gpt-4o-mini, claude-3-5-sonnet)
- **Tool Calling**: LLM automatically invokes conversion tools when needed
- **Contextual Responses**: Provides explanations and step-by-step reasoning

### 6. API Endpoints

#### Health Check
- **GET `/health`**: Returns system status and component availability
- Useful for monitoring and debugging

#### Chat
- **POST `/api/chat`**: Main chat endpoint
- Accepts messages and conversation history
- Returns LLM response with updated conversation history

## Example Queries

### Simple Conversions
- "Convert 10 miles to kilometers"
- "What is 100 pounds in kilograms?"
- "How many seconds are in 2 hours?"

### Complex Queries
- "Convert 5 feet 10 inches to meters"
- "What's the equivalent of 100 km/h in mph?"
- "Convert 1.5 acres to square meters"

### Temperature Conversions
- "What's 32 degrees Fahrenheit in Celsius?"
- "Convert 0 Kelvin to Celsius"
- "How many degrees Rankine is 100 Celsius?"

## User Interface Features

### Chat Interface
- **Message Display**: User messages on right (blue), assistant on left (gray)
- **Auto-scrolling**: Automatically scrolls to latest message
- **Loading Indicator**: Shows "Thinking..." while processing
- **Input Validation**: Prevents empty messages
- **Error Display**: Shows errors in conversation when they occur

### Responsive Design
- **Tailwind CSS**: Modern, responsive styling
- **Mobile-friendly**: Works on various screen sizes
- **Clean Layout**: Minimal, focused interface

## Technical Capabilities

### Unit Recognition
The system recognizes units in various formats:
- Full names: "meters", "kilometers", "pounds"
- Abbreviations: "m", "km", "lb"
- Plural forms: "meters" or "meter"
- Case-insensitive: "MILES" or "miles"

### Error Handling
- **Invalid Units**: Clear error messages for unsupported units
- **Network Issues**: User-friendly connection error messages
- **LLM Errors**: Graceful handling of LLM service failures
- **Conversion Errors**: Detailed error messages for conversion failures

### Performance
- **Fast Conversions**: Rust implementation ensures sub-millisecond conversion times
- **Efficient API**: RESTful design with minimal overhead
- **Caching Ready**: Architecture supports future caching implementation

## Limitations

### Current Limitations
1. **Single Conversion per Query**: One conversion per message (though LLM can handle follow-ups)
2. **No Unit Validation**: LLM must correctly identify units (no pre-validation)
3. **No History Persistence**: Conversation history is not saved between sessions
4. **No User Accounts**: No authentication or user-specific features
5. **MCP Physics**: Physics MCP integration is not yet implemented (placeholder exists)

### Known Limitations

### Current Limitations
1. **Single Conversion per Query**: One conversion per message (though LLM can handle follow-ups)
2. **No Unit Validation**: LLM must correctly identify units (no pre-validation)
3. **No History Persistence**: Conversation history is not saved between sessions
4. **No User Accounts**: No authentication or user-specific features
5. **MCP Physics**: Physics MCP integration is not yet implemented (placeholder exists)
6. **Rust Module**: Must be built manually with `maturin develop` before use
7. **Streaming**: Not yet implemented; responses are returned all at once

### Known Issues
- Rust module must be built manually with `maturin develop`
- Some edge cases in unit name recognition may require clarification
- Large numbers may have precision limitations in display
- Conversation history is handled per-request; could be enhanced with session management

## Future Enhancements

### Planned Features
- **Multi-step Conversions**: Convert through intermediate units
- **Unit Validation**: Pre-validate units before conversion
- **Conversion History**: Save and recall previous conversions
- **Unit Suggestions**: Suggest similar units when typos are detected
- **Batch Conversions**: Convert multiple values in one query
- **Custom Units**: Allow users to define custom conversion factors

### Potential Integrations
- **Physics MCP**: Advanced physics calculations and symbolic math
- **Unit Database**: Expandable unit database with more obscure units
- **Currency Conversion**: Add currency conversion capabilities
- **Historical Units**: Support for historical measurement units

## Usage Examples

### Basic Usage
1. Open the application in a web browser
2. Type a conversion question in the chat input
3. Press Enter or click Send
4. View the response with the conversion result

### API Usage
```bash
curl -X POST http://localhost:8000/api/chat \
  -H "Content-Type: application/json" \
  -d '{
    "message": "Convert 5 miles to kilometers",
    "conversation_history": []
  }'
```

### Health Check
```bash
curl http://localhost:8000/health
```

## Configuration

### Environment Variables
- `LLM_PROVIDER`: "openai" or "anthropic"
- `LLM_MODEL`: Model name (e.g., "gpt-4o-mini")
- `OPENAI_API_KEY`: OpenAI API key (if using OpenAI)
- `ANTHROPIC_API_KEY`: Anthropic API key (if using Anthropic)

### Frontend Configuration
- `API_BASE`: Backend API URL (default: "http://localhost:8000")

## Support and Documentation

- **Setup Guide**: See `SETUP.md` for installation instructions
- **Architecture**: See `docs/architecture.md` for system design
- **API Documentation**: FastAPI provides automatic OpenAPI docs at `/docs`


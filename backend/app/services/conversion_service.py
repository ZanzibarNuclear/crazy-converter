"""
Conversion service that wraps Rust conversion functions and provides them as tools for Pydantic AI.
"""
from typing import Literal
from pydantic import BaseModel
from pydantic_ai import Tool

# Import Rust conversion functions
RUST_MODULE_AVAILABLE = False
try:
    from converterator_rust import (
        convert_time,
        convert_length,
        convert_area,
        convert_volume,
        convert_mass,
        convert_speed,
        convert_acceleration,
        convert_force,
        convert_pressure,
        convert_energy,
        convert_power,
        convert_momentum,
        convert_torque,
        convert_temperature,
    )
    RUST_MODULE_AVAILABLE = True
except ImportError as e:
    # Fallback if Rust module not installed yet
    RUST_MODULE_AVAILABLE = False
    _import_error = str(e)
    def _placeholder(*args, **kwargs):
        raise ImportError(
            f"Rust conversion module not installed. "
            f"Run 'maturin develop' in the rust/ directory. "
            f"Original error: {_import_error}"
        )
    convert_time = convert_length = convert_area = convert_volume = convert_mass = _placeholder
    convert_speed = convert_acceleration = convert_force = convert_pressure = _placeholder
    convert_energy = convert_power = convert_momentum = convert_torque = _placeholder
    convert_temperature = _placeholder


class ConversionResult(BaseModel):
    """Result of a unit conversion."""
    value: float
    from_unit: str
    to_unit: str
    result: float
    category: str


def convert_unit(
    value: float,
    from_unit: str,
    to_unit: str,
    category: Literal[
        "time", "length", "area", "volume", "mass",
        "speed", "acceleration", "force", "pressure",
        "energy", "power", "momentum", "torque", "temperature"
    ]
) -> ConversionResult:
    """
    Convert a value from one unit to another within a specific category.
    
    Args:
        value: The numerical value to convert
        from_unit: The source unit (e.g., "miles", "kg", "seconds")
        to_unit: The target unit (e.g., "kilometers", "pounds", "minutes")
        category: The category of units (time, length, area, volume, mass, speed, acceleration, force, pressure, energy, power, momentum, torque)
    
    Returns:
        A ConversionResult with the converted value and metadata
    """
    # Map category to conversion function
    conversion_functions = {
        "time": convert_time,
        "length": convert_length,
        "area": convert_area,
        "volume": convert_volume,
        "mass": convert_mass,
        "speed": convert_speed,
        "acceleration": convert_acceleration,
        "force": convert_force,
        "pressure": convert_pressure,
        "energy": convert_energy,
        "power": convert_power,
        "momentum": convert_momentum,
        "torque": convert_torque,
        "temperature": convert_temperature,
    }
    
    if category not in conversion_functions:
        raise ValueError(f"Unknown category: {category}. Must be one of {list(conversion_functions.keys())}")
    
    convert_func = conversion_functions[category]
    
    try:
        result_value = convert_func(value, from_unit, to_unit)
        return ConversionResult(
            value=value,
            from_unit=from_unit,
            to_unit=to_unit,
            result=result_value,
            category=category
        )
    except ImportError as e:
        # Re-raise ImportError as-is (from Rust module not being available)
        raise
    except ValueError as e:
        # Re-raise ValueError as-is (from invalid units)
        raise
    except Exception as e:
        raise ValueError(f"Conversion failed: {str(e)}")


# List of all available tools
conversion_tools = [Tool(convert_unit)]


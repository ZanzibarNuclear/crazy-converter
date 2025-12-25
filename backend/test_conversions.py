#!/usr/bin/env python3
"""
Test script to verify Rust conversion functions work correctly.
Run this after building the Rust module with 'maturin develop'.
"""
import sys

def test_rust_module():
    """Test that the Rust module can be imported and basic conversions work."""
    try:
        from converterator_rust import (
            convert_time, convert_length, convert_area, convert_volume,
            convert_mass, convert_speed, convert_acceleration, convert_force,
            convert_pressure, convert_energy, convert_power, convert_momentum,
            convert_torque, convert_temperature,
        )
        print("✓ Rust module imported successfully")
    except ImportError as e:
        print(f"✗ Failed to import Rust module: {e}")
        print("  Run 'maturin develop' in the rust/ directory to build the module.")
        return False
    
    # Test cases: (category, value, from_unit, to_unit, expected_approx)
    test_cases = [
        ("time", 60, "seconds", "minutes", 1.0),
        ("length", 1, "kilometer", "meters", 1000.0),
        ("area", 1, "acre", "square_meters", 4046.86),
        ("volume", 1, "liter", "milliliters", 1000.0),
        ("mass", 1, "kilogram", "grams", 1000.0),
        ("speed", 1, "meter_per_second", "kilometers_per_hour", 3.6),
        ("acceleration", 9.8, "meters_per_second_squared", "feet_per_second_squared", 32.15),
        ("force", 1, "newton", "pounds_force", 0.2248),
        ("pressure", 1, "pascal", "psi", 0.000145038),
        ("energy", 1, "joule", "calories", 0.239),
        ("power", 1, "watt", "horsepower", 0.001341),
        ("momentum", 1, "kilogram_meter_per_second", "pound_foot_per_second", 7.233),
        ("torque", 1, "newton_meter", "pound_foot", 0.737562),
        ("temperature", 0, "celsius", "fahrenheit", 32.0),
    ]
    
    passed = 0
    failed = 0
    
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
    
    print("\nTesting conversions:")
    print("-" * 60)
    
    for category, value, from_unit, to_unit, expected in test_cases:
        try:
            func = conversion_functions[category]
            result = func(value, from_unit, to_unit)
            
            # Check if result is within 5% of expected (allowing for rounding differences)
            tolerance = abs(expected * 0.05)
            if abs(result - expected) <= tolerance:
                print(f"✓ {category}: {value} {from_unit} → {result:.4f} {to_unit} (expected ~{expected})")
                passed += 1
            else:
                print(f"✗ {category}: {value} {from_unit} → {result:.4f} {to_unit} (expected ~{expected})")
                failed += 1
        except Exception as e:
            print(f"✗ {category}: Error - {e}")
            failed += 1
    
    print("-" * 60)
    print(f"\nResults: {passed} passed, {failed} failed")
    
    if failed == 0:
        print("✓ All conversion tests passed!")
        return True
    else:
        print("✗ Some conversion tests failed.")
        return False


if __name__ == "__main__":
    success = test_rust_module()
    sys.exit(0 if success else 1)


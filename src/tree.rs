/// Auto-generated decision tree for binary/text classification.
///
/// Ported from the Python decision tree in binaryornot/tree.py.
/// Takes the 24-element feature vector from `compute_features()`.
/// Returns `true` for binary, `false` for text.

pub fn is_binary(features: &[f64; 24]) -> bool {
    if features[1] <= 0.000977 {
        if features[17] <= 0.000977 {
            if features[4] <= 0.5 {
                if features[22] <= 0.5 {
                    if features[7] <= 1.953445 {
                        return true;
                    } else if features[7] <= 2.749470 {
                        return false;
                    } else if features[14] <= 0.5 {
                        return true;
                    } else {
                        return true;
                    }
                } else {
                    return false;
                }
            } else {
                return false;
            }
        } else {
            if features[4] <= 0.5 {
                if features[17] <= 0.063636 {
                    if features[7] <= 2.021329 {
                        return true;
                    } else if features[13] <= 0.5 {
                        return false;
                    } else {
                        return false;
                    }
                } else {
                    if features[7] <= 3.047898 {
                        return false;
                    } else if features[2] <= 0.758706 {
                        if features[17] <= 0.134848 {
                            if features[3] <= 0.633612 {
                                if features[7] <= 3.851786 {
                                    return false;
                                } else if features[3] <= 0.483333 {
                                    return false;
                                } else if features[17] <= 0.129167 {
                                    return true;
                                } else {
                                    return false;
                                }
                            } else if features[2] <= 0.145455 {
                                return false;
                            } else {
                                return true;
                            }
                        } else if features[19] <= 0.5 {
                            if features[7] <= 3.572021 {
                                if features[3] <= 0.49 {
                                    if features[2] <= 0.651515 {
                                        return false;
                                    } else {
                                        return true;
                                    }
                                } else if features[7] <= 3.370112 {
                                    return true;
                                } else if features[3] <= 0.651515 {
                                    if features[17] <= 0.318182 {
                                        return true;
                                    } else {
                                        return true;
                                    }
                                } else {
                                    return true;
                                }
                            } else if features[2] <= 0.615079 {
                                if features[20] <= 0.5 {
                                    if features[13] <= 0.5 {
                                        return true;
                                    } else {
                                        return true;
                                    }
                                } else if features[17] <= 0.176923 {
                                    return false;
                                } else {
                                    return true;
                                }
                            } else if features[17] <= 0.252381 {
                                return false;
                            } else {
                                return true;
                            }
                        } else if features[7] <= 3.889752 {
                            return true;
                        } else {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
            } else {
                return false;
            }
        }
    } else {
        if features[0] <= 0.163978 {
            if features[4] <= 0.5 {
                if features[7] <= 3.264621 {
                    if features[17] <= 0.022678 {
                        if features[2] <= 0.000977 {
                            if features[5] <= 0.001953 {
                                return false;
                            } else {
                                return true;
                            }
                        } else {
                            return true;
                        }
                    } else if features[1] <= 0.095455 {
                        if features[17] <= 0.306818 {
                            if features[1] <= 0.055728 {
                                return false;
                            } else {
                                return false;
                            }
                        } else {
                            return true;
                        }
                    } else {
                        return false;
                    }
                } else if features[10] <= 0.5 {
                    if features[2] <= 0.455534 {
                        if features[3] <= 0.79057 {
                            if features[1] <= 0.485714 {
                                if features[7] <= 3.540884 {
                                    if features[7] <= 3.528408 {
                                        if features[2] <= 0.316667 {
                                            if features[2] <= 0.286364 {
                                                if features[17] <= 0.095455 {
                                                    return true;
                                                } else {
                                                    return true;
                                                }
                                            } else if features[13] <= 0.5 {
                                                return false;
                                            } else {
                                                return true;
                                            }
                                        } else {
                                            return true;
                                        }
                                    } else {
                                        return false;
                                    }
                                } else if features[2] <= 0.446657 {
                                    return true;
                                } else if features[17] <= 0.066638 {
                                    if features[2] <= 0.450099 {
                                        return false;
                                    } else {
                                        return true;
                                    }
                                } else {
                                    return true;
                                }
                            } else if features[2] <= 0.446657 {
                                return true;
                            } else if features[17] <= 0.066638 {
                                if features[2] <= 0.450099 {
                                    return false;
                                } else {
                                    return true;
                                }
                            } else {
                                return true;
                            }
                        } else {
                            return false;
                        }
                    } else if features[3] <= 0.425959 {
                        if features[3] <= 0.394552 {
                            if features[7] <= 4.209417 {
                                if features[7] <= 4.175213 {
                                    if features[3] <= 0.100962 {
                                        return false;
                                    } else if features[3] <= 0.248047 {
                                        return true;
                                    } else if features[17] <= 0.106443 {
                                        return false;
                                    } else {
                                        return true;
                                    }
                                } else {
                                    return false;
                                }
                            } else if features[23] <= 0.5 {
                                return true;
                            } else {
                                return true;
                            }
                        } else if features[17] <= 0.082906 {
                            return false;
                        } else if features[1] <= 0.025 {
                            return false;
                        } else if features[7] <= 3.520764 {
                            return false;
                        } else if features[1] <= 0.1225 {
                            return true;
                        } else if features[0] <= 0.033333 {
                            return false;
                        } else {
                            return true;
                        }
                    } else if features[17] <= 0.098599 {
                        if features[1] <= 0.012902 {
                            return true;
                        } else if features[23] <= 0.5 {
                            return false;
                        } else if features[3] <= 0.458087 {
                            return true;
                        } else {
                            return true;
                        }
                    } else if features[7] <= 4.453697 {
                        if features[3] <= 0.472136 {
                            if features[17] <= 0.121324 {
                                return false;
                            } else if features[17] <= 0.302885 {
                                if features[7] <= 3.539612 {
                                    return false;
                                } else {
                                    return true;
                                }
                            } else {
                                return false;
                            }
                        } else {
                            return true;
                        }
                    } else {
                        return true;
                    }
                } else if features[14] <= 0.5 {
                    return false;
                } else {
                    return false;
                }
            } else if features[1] <= 0.45 {
                if features[1] <= 0.006212 {
                    return true;
                } else {
                    return false;
                }
            } else {
                return true;
            }
        } else {
            if features[23] <= 0.5 {
                if features[3] <= 0.082843 {
                    if features[6] <= 0.282534 {
                        return false;
                    } else if features[7] <= 3.088801 {
                        if features[0] <= 0.822852 {
                            if features[2] <= 0.291667 {
                                return false;
                            } else if features[1] <= 0.521739 {
                                return false;
                            } else {
                                return true;
                            }
                        } else if features[16] <= 0.5 {
                            return true;
                        } else {
                            return true;
                        }
                    } else if features[5] <= 0.140625 {
                        return false;
                    } else {
                        return true;
                    }
                } else if features[17] <= 0.291667 {
                    if features[7] <= 0.698576 {
                        return true;
                    } else if features[7] <= 5.613847 {
                        if features[2] <= 0.361264 {
                            if features[20] <= 0.5 {
                                return false;
                            } else if features[14] <= 0.5 {
                                if features[0] <= 0.379808 {
                                    return false;
                                } else {
                                    return true;
                                }
                            } else {
                                return false;
                            }
                        } else if features[2] <= 0.408333 {
                            if features[1] <= 0.26 {
                                return false;
                            } else {
                                return true;
                            }
                        } else {
                            return false;
                        }
                    } else if features[17] <= 0.012485 {
                        return false;
                    } else {
                        return true;
                    }
                } else {
                    return true;
                }
            } else if features[20] <= 0.5 {
                return true;
            } else {
                return true;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plain_text_classification() {
        let features = [
            0.0,    // null_ratio
            0.0,    // control_ratio
            1.0,    // printable_ascii_ratio
            0.0,    // high_byte_ratio
            1.0,    // utf8_valid
            0.0,    // even_null_ratio
            0.0,    // odd_null_ratio
            4.0,    // entropy
            0.0, 0.0, 0.0, 0.0, 0.0, // BOM flags
            0.0, 0.0, 0.0, 0.0, // UTF-16/32
            1.0,    // longest_printable_run
            0.0, 0.0, 0.0, 0.0, 0.0, // CJK
            0.0,    // has_magic
        ];
        assert!(!is_binary(&features));
    }

    #[test]
    fn test_high_null_binary_classification() {
        // High null ratio + high entropy + no valid encoding → binary
        // Trace: control > threshold → null > 0.16 → has_magic ≤ 0.5 → high_byte > 0.08
        //   → run > 0.29 → return true
        let features = [
            0.5,    // null_ratio
            0.1,    // control_ratio
            0.05,   // printable_ascii_ratio
            0.4,    // high_byte_ratio
            0.0,    // utf8_valid
            0.25,   // even_null_ratio
            0.25,   // odd_null_ratio
            5.0,    // entropy
            0.0, 0.0, 0.0, 0.0, 0.0, // BOM flags
            0.0, 0.0, 0.0, 0.0, // UTF-16/32
            0.35,   // longest_printable_run (above 0.291667 threshold)
            0.0, 0.0, 0.0, 0.0, 0.0, // CJK
            0.0,    // has_magic
        ];
        assert!(is_binary(&features));
    }
}

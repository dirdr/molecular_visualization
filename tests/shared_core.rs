use molecular_visualization::{arcball::ArcballControl, molecule_data::MoleculeData, shaders};

#[test]
fn all_supplied_molecules_have_finite_geometry() {
    for entry in std::fs::read_dir("resources/pdb").unwrap() {
        let path = entry.unwrap().path();
        if path.extension().and_then(|value| value.to_str()) != Some("pdb") {
            continue;
        }
        let molecule = MoleculeData::from_pdb(&std::fs::read_to_string(&path).unwrap())
            .unwrap_or_else(|error| panic!("{}: {error}", path.display()));
        assert!(!molecule.atoms.is_empty(), "{}", path.display());
        assert!(molecule.radius.is_finite() && molecule.radius > 0.0);
        assert!(molecule.scale_factor > 0.0 && molecule.scale_factor <= 1.0);
        let center = molecule.atoms.iter().fold([0.0; 3], |mut total, atom| {
            for (i, value) in atom.instance_pos.iter().enumerate() {
                total[i] += value;
            }
            total
        });
        assert!(center
            .iter()
            .all(|sum| (sum / molecule.atoms.len() as f32).abs() < 0.001));
        for bond in &molecule.bonds {
            assert_ne!(bond.instance_start_pos, bond.instance_end_pos);
            assert!(bond
                .instance_start_pos
                .iter()
                .chain(bond.instance_end_pos.iter())
                .all(|v| v.is_finite()));
        }
    }
}

#[test]
fn methane_preserves_original_colors_radii_and_bonds() {
    let text = include_str!("../resources/pdb/methane.pdb");
    let molecule = MoleculeData::from_pdb(text).unwrap();
    assert_eq!(molecule.atoms.len(), 5);
    assert_eq!(molecule.bonds.len(), 4);
    assert_eq!(molecule.atoms[0].instance_color, [0.565, 0.565, 0.565, 1.0]);
    assert_eq!(molecule.atoms[0].instance_radius, 1.7 * 0.25);
    let without_bonds = text
        .lines()
        .filter(|line| !line.starts_with("CONECT"))
        .collect::<Vec<_>>()
        .join("\n");
    assert!(MoleculeData::from_pdb(&without_bonds)
        .unwrap()
        .bonds
        .is_empty());
    assert!(MoleculeData::from_pdb("not a molecule").is_err());
    assert!(MoleculeData::from_pdb("").is_err());
}

#[test]
fn original_arcball_handles_drag_and_reset() {
    let mut control = ArcballControl::new(700.0, 450.0);
    let initial = control.get_rotation_matrix();
    control.mouse_down(200.0, 200.0);
    control.mouse_move(500.0, 100.0);
    control.mouse_up();
    assert_ne!(control.get_rotation_matrix(), initial);
    assert!((control.get_rotation().norm() - 1.0).abs() < 0.0001);
    control.resize(0.0, 0.0);
    control.mouse_down(0.0, 0.0);
    control.mouse_move(0.0, 0.0);
    assert!(control.get_rotation_matrix().iter().all(|v| v.is_finite()));
    control.reset();
    assert_eq!(control.get_rotation_matrix(), initial);
}

#[test]
fn webgl_adaptation_only_changes_the_shader_header() {
    for source in [
        shaders::SPHERE_VERTEX,
        shaders::SPHERE_FRAGMENT,
        shaders::CYLINDER_VERTEX,
        shaders::CYLINDER_FRAGMENT,
    ] {
        let adapted = shaders::webgl_source(source);
        assert!(adapted.starts_with("#version 300 es\nprecision highp float;\n"));
        assert_eq!(
            adapted.split_once("precision highp float;\n").unwrap().1,
            source.strip_prefix("#version 410 core\n").unwrap()
        );
    }
}

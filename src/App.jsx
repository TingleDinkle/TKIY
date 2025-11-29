import React, { useState, useRef, useEffect } from 'react';
import { Canvas, useFrame, useThree } from '@react-three/fiber';
import { useGLTF } from '@react-three/drei';
import * as THREE from 'three';
import Editor from 'react-simple-code-editor';
import { highlight, languages } from 'prismjs/components/prism-core';
import 'prismjs/components/prism-clike';
import 'prismjs/components/prism-javascript';
import 'prismjs/themes/prism.css';

// Import WASM
import init, { YellowWebInterpreter } from './pkg/the_yellow_compiler';

function MinecraftEditor({ interpreter, onReset }) {
  const [code, setCode] = useState(
    `# Act I: The Yellow Sign
mask truth -> 42;
echo(truth);`
  );
  const [output, setOutput] = useState("The void awaits your command...");
  const [sanity, setSanity] = useState(100);
  const [showGlitch, setShowGlitch] = useState(false);

  const runCode = () => {
    if (!interpreter) {
      setOutput("Error: The Yellow Interpreter has not yet manifested (WASM not loaded).");
      return;
    }
    
    try {
      console.log("Executing Code:\n", code);
      const result = interpreter.run_code(code);
      setOutput(result);

      const newSanity = interpreter.get_sanity();
      setSanity(newSanity);

      // The Crossroads Glitch
      if (newSanity < 20 && Math.random() > 0.7) {
        setShowGlitch(true);
        setTimeout(() => setShowGlitch(false), 200);
      }
    } catch (e) {
      console.error(e);
      setOutput("Runtime Horror: " + e);
    }
  };

  // Helper for dynamic styles
  const getSanityEffects = () => {
    const effects = {};
    
    if (sanity < 80) {
      effects.filter = 'blur(0.5px)';
    }
    
    if (sanity < 50) {
      effects.color = '#550000';
      effects.transform = `rotate(${Math.random() - 0.5}deg)`;
    }
    
    if (sanity < 20) {
      effects.filter = 'contrast(1.5) invert(0.1)';
      effects.animation = 'shake 0.1s infinite';
    }
    
    return effects;
  };

  const getSanityColor = () => {
    if (sanity > 70) return '#2e7d32';
    if (sanity > 30) return '#f9a825';
    return '#c62828';
  };

  const styles = {
    container: {
      width: '90vw',
      maxWidth: '1000px',
      height: '80vh',
      maxHeight: '90vh',
      boxSizing: 'border-box',
      backgroundColor: '#C6B296',
      border: '8px solid #5A3E25',
      boxShadow: '12px 12px 0px rgba(0,0,0,0.5)',
      fontFamily: '"Courier New", Courier, monospace',
      display: 'flex',
      flexDirection: 'column',
      padding: '20px',
      position: 'relative',
      transition: 'all 0.5s ease',
      ...getSanityEffects(), // Apply dynamic effects
    },
    header: {
      textAlign: 'center',
      fontSize: '2rem',
      color: '#3e2723',
      marginBottom: '20px',
      textTransform: 'uppercase',
      letterSpacing: '4px',
      fontWeight: 'bold',
      textShadow: '2px 2px 0px #A08B70',
      position: 'relative',
    },
    sanityMeter: {
      position: 'absolute',
      right: 0,
      top: 0,
      fontSize: '1rem',
      color: getSanityColor(),
      border: `2px solid ${getSanityColor()}`,
      padding: '5px 10px',
      backgroundColor: 'rgba(0,0,0,0.1)',
      fontFamily: '"Fira code", monospace',
    },
    bookBody: {
      display: 'flex',
      flex: 1,
      gap: '20px',
      borderTop: '2px solid #5A3E25',
      borderBottom: '2px solid #5A3E25',
      padding: '20px 0',
      minHeight: 0,
    },
    page: {
      flex: 1,
      display: 'flex',
      flexDirection: 'column',
      position: 'relative',
      minHeight: 0,
    },
    leftPage: {
      borderRight: '2px solid #A08B70',
      paddingRight: '20px',
    },
    rightPage: {
      paddingLeft: '20px',
      color: '#800000',
    },
    label: {
      fontSize: '1.2rem',
      marginBottom: '10px',
      color: '#5A3E25',
      fontWeight: 'bold',
      textDecoration: 'underline',
    },
    editorContainer: {
      flex: 1,
      backgroundColor: 'rgba(255, 255, 255, 0.2)',
      borderRadius: '4px',
      overflow: 'auto',
      border: '1px dashed #A08B70',
      position: 'relative',
    },
    outputContainer: {
      flex: 1,
      overflow: 'auto',
      whiteSpace: 'pre-wrap',
      lineHeight: '1.5',
      fontStyle: 'italic',
    },
    button: {
      marginTop: '20px',
      alignSelf: 'center',
      padding: '15px 40px',
      backgroundColor: '#8d6e63',
      color: '#fff',
      border: '4px solid #3e2723',
      fontSize: '1.2rem',
      fontFamily: 'inherit',
      cursor: 'pointer',
      boxShadow: '4px 4px 0px #3e2723',
      transition: 'all 0.1s',
      textTransform: 'uppercase',
      letterSpacing: '2px',
    },
    glitchOverlay: {
      position: 'fixed',
      top: 0,
      left: 0,
      width: '100vw',
      height: '100vh',
      backgroundColor: 'black',
      display: 'flex',
      justifyContent: 'center',
      alignItems: 'center',
      color: 'red',
      fontSize: '5rem',
      fontFamily: 'monospace',
      zIndex: 9999,
      pointerEvents: 'none',
      textShadow: '5px 0px 0px yellow',
    }
  };

  return (
    <>
      <style>
        {`
          @keyframes shake {
            0% { transform: translate(1px, 1px) rotate(0deg); }
            10% { transform: translate(-1px, -2px) rotate(-1deg); }
            20% { transform: translate(-3px, 0px) rotate(1deg); }
            30% { transform: translate(3px, 2px) rotate(0deg); }
            40% { transform: translate(1px, -1px) rotate(1deg); }
            50% { transform: translate(-1px, 2px) rotate(-1deg); }
            60% { transform: translate(-3px, 1px) rotate(0deg); }
            70% { transform: translate(3px, 1px) rotate(-1deg); }
            80% { transform: translate(-1px, -1px) rotate(1deg); }
            90% { transform: translate(1px, 2px) rotate(0deg); }
            100% { transform: translate(1px, -2px) rotate(-1deg); }
          }
        `}
      </style>
      <div style={styles.container}>
        {showGlitch && (
          <div style={styles.glitchOverlay}>
            DON'T TURN LEFT
          </div>
        )}

        <div style={styles.header}>
          TKIY Code Editor
          <div style={styles.sanityMeter}>Sanity: {sanity.toFixed(1)}%</div>
        </div>
        
        <div style={styles.bookBody}>
          {/* Left Page: Input */}
          <div style={{ ...styles.page, ...styles.leftPage }}>
            <div style={styles.label}>Inscription</div>
            <div style={styles.editorContainer}>
              <Editor
                value={code}
                onValueChange={code => setCode(code)}
                highlight={code => highlight(code, languages.js)}
                padding={10}
                style={{
                  fontFamily: '"Fira code", "Fira Mono", monospace',
                  fontSize: 14,
                  backgroundColor: 'transparent',
                  height: '100%',
                }}
              />
            </div>
          </div>

          {/* Right Page: Output */}
          <div style={{ ...styles.page, ...styles.rightPage }}>
            <div style={styles.label}>Manifestation</div>
            <div style={styles.outputContainer}>
              {output}
            </div>
          </div>
        </div>

        <button 
          style={styles.button} 
          onClick={runCode}
          onMouseDown={(e) => {
            e.currentTarget.style.transform = 'translate(2px, 2px)';
            e.currentTarget.style.boxShadow = '2px 2px 0px #3e2723';
          }}
          onMouseUp={(e) => {
            e.currentTarget.style.transform = 'translate(0, 0)';
            e.currentTarget.style.boxShadow = '4px 4px 0px #3e2723';
          }}
        >
          SIGN
        </button>
        
        {sanity < 10 && (
          <button
            style={{ ...styles.button, backgroundColor: '#b71c1c', marginTop: '10px', fontSize: '0.8rem' }}
            onClick={onReset}
          >
            RESET REALITY
          </button>
        )}
      </div>
    </>
  );
}

function YellowMist({ isOpening, onFullMist }) {
  const { scene } = useThree();
  const fogRef = useRef();

  useEffect(() => {
    // Initialize fog: Black, low density
    const fog = new THREE.FogExp2('#000000', 0.02);
    scene.fog = fog;
    fogRef.current = fog;

    return () => {
      scene.fog = null;
    };
  }, [scene]);

  useFrame((state, delta) => {
    if (!fogRef.current) return;

    if (isOpening) {
      // Interpolate color to King in Yellow Gold (#FCD200)
      // We use a slight lerp factor for the color transition
      fogRef.current.color.lerp(new THREE.Color('#FCD200'), delta * 0.5);

      // Rapidly increase density
      fogRef.current.density += delta * 0.3;

      // Trigger callback when fog is thick enough
      if (fogRef.current.density > 0.8) {
        onFullMist();
      }
    }
  });

  return null;
}

function Grimoire({ isOpening, onClick }) {
  const groupRef = useRef();
  const { scene } = useGLTF('/book.glb');

  useFrame((state, delta) => {
    if (!groupRef.current) return;

    const time = state.clock.getElapsedTime();

    if (isOpening) {
      // Violent spin
      groupRef.current.rotation.y += delta * 10; 
      groupRef.current.rotation.x += delta * 5;
      
      // Rise upward
      groupRef.current.position.y += delta * 2;
    } else {
      // Gentle idle animation
      groupRef.current.position.y = Math.sin(time) * 0.2 - 2; // -2 offset included here
      groupRef.current.rotation.y = time * 0.1;
    }
  });

  return (
    <group ref={groupRef} onClick={onClick} onPointerOver={() => document.body.style.cursor = 'pointer'} onPointerOut={() => document.body.style.cursor = 'auto'}> 
       <primitive 
        object={scene} 
        scale={20} 
        position={[0, 0, 0]}
      />
      {/* Debug Mesh: Red Wireframe Box to visualize position if GLB fails */}
      <mesh>
        <boxGeometry args={[2, 3, 0.5]} />
        <meshBasicMaterial color="red" wireframe />
      </mesh>
    </group>
  );
}

function Temple() {
  const pillarGroupRef = useRef();

  useFrame((state, delta) => {
    if (pillarGroupRef.current) {
      // Slow, unsettling rotation of the pillar ring
      pillarGroupRef.current.rotation.y += delta * 0.02;
    }
  });

  // Pillars
  const pillarCount = 12;
  const radius = 20;
  const pillars = Array.from({ length: pillarCount }).map((_, i) => {
    const angle = (i / pillarCount) * Math.PI * 2;
    const x = Math.cos(angle) * radius;
    const z = Math.sin(angle) * radius;
    return (
      <mesh key={i} position={[x, 10, z]}>
        <cylinderGeometry args={[1, 1, 30, 16]} />
        <meshStandardMaterial color="#333" roughness={0.9} />
      </mesh>
    );
  });

  return (
    <group>
      {/* Floor */}
      <mesh rotation={[-Math.PI / 2, 0, 0]} position={[0, -5, 0]}>
        <planeGeometry args={[100, 100]} />
        <meshStandardMaterial 
          color="#1a1a1a" 
          roughness={0.05} 
          metalness={0.5} 
        />
      </mesh>
      {/* Rotating Pillars */}
      <group ref={pillarGroupRef}>
        {pillars}
      </group>
    </group>
  );
}

export default function App() {
  const [isOpening, setIsOpening] = useState(false);
  const [mode, setMode] = useState('3D');
  const [interpreter, setInterpreter] = useState(null);

  // Initialize WASM
  useEffect(() => {
    const loadWasm = async () => {
      try {
        await init();
        const interp = new YellowWebInterpreter();
        setInterpreter(interp);
        console.log("The Yellow Interpreter has been summoned.");
      } catch (err) {
        console.error("Failed to summon the interpreter:", err);
      }
    };
    loadWasm();
  }, []);

  const resetInterpreter = () => {
    const interp = new YellowWebInterpreter();
    setInterpreter(interp);
  };

  if (mode === 'EDITOR') {
    return (
      <div style={{
        width: '100vw',
        height: '100vh',
        backgroundColor: '#FCD200',
        display: 'flex',
        justifyContent: 'center',
        alignItems: 'center',
      }}>
        <MinecraftEditor interpreter={interpreter} onReset={resetInterpreter} />
      </div>
    );
  }

  return (
    <div style={{ width: '100vw', height: '100vh' }}>
      <Canvas camera={{ position: [0, 0, 15], fov: 60 }}>
        {/* Atmosphere */}
        <color attach="background" args={['#050505']} />
        <YellowMist isOpening={isOpening} onFullMist={() => setMode('EDITOR')} />
        
        {/* Lighting */}
        <ambientLight intensity={0.5} />
        <pointLight 
          position={[2, 2, 2]} 
          intensity={1.5} 
          color="#FCD200" 
          distance={10}
          decay={2}
        />

        {/* The Artifact */}
        <Grimoire isOpening={isOpening} onClick={() => setIsOpening(true)} />

        {/* Temple of the King */}
        <Temple />
      </Canvas>
    </div>
  );
}

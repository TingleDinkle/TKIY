import React, { useState, useRef, useEffect } from 'react';
import { Canvas, useFrame, useThree } from '@react-three/fiber';
import { useGLTF, Stars } from '@react-three/drei';
import * as THREE from 'three';
import Editor from 'react-simple-code-editor';
import { highlight, languages } from 'prismjs/components/prism-core';
import 'prismjs/components/prism-clike';
import 'prismjs/components/prism-javascript';
import 'prismjs/themes/prism.css';

// Import WASM
import init, { YellowWebInterpreter } from './pkg/the_yellow_compiler';

function MinecraftEditor({ interpreter }) {
  const [code, setCode] = useState(
    `# Act I: The Yellow Sign
mask truth -> 42;
echo(truth);`
  );
  const [output, setOutput] = useState("The void awaits your command...");

  const runCode = () => {
    if (!interpreter) {
      setOutput("Error: The Yellow Interpreter has not yet manifested (WASM not loaded).");
      return;
    }
    
    try {
      console.log("Executing Code:\n", code);
      const result = interpreter.run_code(code);
      setOutput(result);
    } catch (e) {
      console.error(e);
      setOutput("Runtime Horror: " + e);
    }
  };

  const styles = {
    container: {
      width: '90vw',
      maxWidth: '1000px',
      height: '80vh',
      backgroundColor: '#C6B296',
      border: '8px solid #5A3E25',
      boxShadow: '12px 12px 0px rgba(0,0,0,0.5)',
      fontFamily: '"Courier New", Courier, monospace',
      display: 'flex',
      flexDirection: 'column',
      padding: '20px',
      position: 'relative',
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
    },
    bookBody: {
      display: 'flex',
      flex: 1,
      gap: '20px',
      borderTop: '2px solid #5A3E25',
      borderBottom: '2px solid #5A3E25',
      padding: '20px 0',
    },
    page: {
      flex: 1,
      display: 'flex',
      flexDirection: 'column',
      position: 'relative',
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
  };

  return (
    <div style={styles.container}>
      <h1 style={styles.header}>TKIY Code Editor</h1>
      
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
                minHeight: '100%',
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
    </div>
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
  const ref = useRef();
  const { scene } = useGLTF('/book.glb');

  useFrame((state, delta) => {
    if (!ref.current) return;

    const time = state.clock.getElapsedTime();

    if (isOpening) {
      // Violent spin
      ref.current.rotation.y += delta * 10; 
      ref.current.rotation.x += delta * 5;
      
      // Rise upward
      ref.current.position.y += delta * 2;
    } else {
      // Gentle idle animation
      ref.current.position.y = Math.sin(time) * 0.2;
      ref.current.rotation.y = time * 0.1;
    }
  });

  return (
    <primitive 
      object={scene} 
      ref={ref} 
      position={[0, 0, 0]} 
      onClick={onClick}
      onPointerOver={() => document.body.style.cursor = 'pointer'}
      onPointerOut={() => document.body.style.cursor = 'auto'}
    />
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
        <MinecraftEditor interpreter={interpreter} />
      </div>
    );
  }

  return (
    <div style={{ width: '100vw', height: '100vh' }}>
      <Canvas camera={{ position: [0, 0, 5], fov: 60 }}>
        {/* Atmosphere */}
        <color attach="background" args={['#050505']} />
        <YellowMist isOpening={isOpening} onFullMist={() => setMode('EDITOR')} />
        
        {/* Lighting */}
        <ambientLight intensity={0.2} />
        <pointLight 
          position={[2, 2, 2]} 
          intensity={1.5} 
          color="#FCD200" 
          distance={10}
          decay={2}
        />

        {/* The Artifact */}
        <Grimoire isOpening={isOpening} onClick={() => setIsOpening(true)} />

        {/* Background Depth */}
        <Stars 
          radius={100} 
          depth={50} 
          count={5000} 
          factor={4} 
          saturation={0} 
          fade 
          speed={1} 
        />
      </Canvas>
    </div>
  );
}
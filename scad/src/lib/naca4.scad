module airfoil(naca=12, L = 100, N = 81, h = 1, open = false)
{
  linear_extrude(height = h)
  polygon(points = airfoil_data(naca, L, N, open)); 
}

function airfoil_data(naca=12, L = 100, N = 81, open = false) = 
  let(Na = naca[0]!=undef?naca:NACA(naca))
  let(A = [.2969, -0.126, -.3516, .2843, open?-0.1015:-0.1036])
  [for (b=[-180:360/(N):179.99]) 
    let (x = (1-cos(b))/2)  
    let(yt = sign(b)*Na[2]/.2*(A*[sqrt(x), x, x*x, x*x*x, x*x*x*x])) 
    Na[0]==0?L*[x, yt]:L*camber(x, yt, Na[0], Na[1], sign(b))];  

function NACA(naca) = 
  let (M = floor(naca/1000))
  let (P = floor((naca-M*1000)/100)) 
  [M/100, P/10, floor(naca-M*1000-P*100)/100];  

function camber(x, y, M, P, upper) = 
  let(yc = (x<P)?M/P/P*(2*P*x-x*x): M/(1-P)/(1-P)*(1 - 2*P + 2*P*x -x*x))
  let(dy = (x<P)?2*M/P/P*(P-x):2*M/(1-P)/(1-P)*(P-x))
  let(th = atan(dy))
  [upper ? x - y*sin(th):x + y*sin(th), upper ? yc + y*cos(th):yc - y*cos(th)];

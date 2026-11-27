// The scene of the DMX-9 spike, ported one to one from egui/src/scene.rs. Throwaway code.
// Vertices are x, y, z, r, g, b, a.

using System.Numerics;

namespace Dmx9;

public sealed class Camera
{
    public float Yaw = 0.6f, Pitch = 0.45f, Dist = 11f, FovY = 50f * MathF.PI / 180f;
    public Vector3 Target = new(0, 1.2f, 0);

    public Vector3 Eye => Target + Dist * new Vector3(MathF.Cos(Pitch) * MathF.Sin(Yaw), MathF.Sin(Pitch), MathF.Cos(Pitch) * MathF.Cos(Yaw));

    private (Vector3 F, Vector3 R, Vector3 U) Basis()
    {
        var f = Vector3.Normalize(Target - Eye);
        var r = Vector3.Normalize(Vector3.Cross(f, Vector3.UnitY));
        return (f, r, Vector3.Cross(r, f));
    }

    /// Column-major projection × view, for glUniformMatrix4fv without transpose.
    public Matrix4x4 ViewProj(float aspect) =>
        Matrix4x4.CreateLookAt(Eye, Target, Vector3.UnitY) * Matrix4x4.CreatePerspectiveFieldOfView(FovY, aspect, 0.1f, 100f);

    public (Vector3 O, Vector3 D) Ray(float nx, float ny, float aspect)
    {
        var (f, r, u) = Basis();
        float th = MathF.Tan(FovY / 2);
        return (Eye, Vector3.Normalize(f + r * (nx * th * aspect) + u * (ny * th)));
    }
}

public static class Scene
{
    public const float RoomX = 4f, RoomZ = 3f, RoomH = 3f;
    public const int Fixtures = 50;
    private const int BeamSegments = 16;
    private const float BeamHalfAngle = 0.14f;

    private static float Axis(Vector3 v, int a) => a == 0 ? v.X : a == 1 ? v.Y : v.Z;

    private static bool WallVisible(Vector3 eye, int axis, float side) =>
        Axis(eye, axis) * side < (axis == 0 ? RoomX : RoomZ);

    public static Vector3? Pick(Camera cam, float nx, float ny, float aspect)
    {
        var (o, d) = cam.Ray(nx, ny, aspect);
        var eye = cam.Eye;
        float best = float.MaxValue;
        Vector3? hit = null;
        void Consider(float t)
        {
            if (t <= 0 || t >= best) return;
            var p = o + d * t;
            const float eps = 1e-3f;
            if (MathF.Abs(p.X) <= RoomX + eps && MathF.Abs(p.Z) <= RoomZ + eps && p.Y >= -eps && p.Y <= RoomH + eps)
            {
                best = t;
                hit = p;
            }
        }
        if (MathF.Abs(d.Y) > 1e-6f) Consider(-o.Y / d.Y);
        foreach (var (axis, limit) in new[] { (0, RoomX), (2, RoomZ) })
            foreach (var side in new[] { -1f, 1f })
                if (WallVisible(eye, axis, side) && MathF.Abs(Axis(d, axis)) > 1e-6f)
                    Consider((side * limit - Axis(o, axis)) / Axis(d, axis));
        return hit;
    }

    public static Vector3 FixturePos(int i) => new(-3.6f + i % 10 * 0.8f, 2.8f, -2.4f + i / 10 * 1.2f);

    public static bool IsSelected(int i) => i % 2 == 0;

    private static void Put(List<float> v, Vector3 p, Vector4 c)
    {
        v.Add(p.X); v.Add(p.Y); v.Add(p.Z); v.Add(c.X); v.Add(c.Y); v.Add(c.Z); v.Add(c.W);
    }

    private static void Quad(List<float> v, Vector3 a, Vector3 b, Vector3 c, Vector3 d, Vector4 col)
    {
        foreach (var p in new[] { a, b, c, a, c, d }) Put(v, p, col);
    }

    private static void Cube(List<float> v, Vector3 c, float h, Vector4 col)
    {
        Vector3 P(float x, float y, float z) => c + new Vector3(x, y, z) * h;
        Vector4 S(float s) => new(col.X * s, col.Y * s, col.Z * s, col.W);
        Quad(v, P(-1, 1, -1), P(1, 1, -1), P(1, 1, 1), P(-1, 1, 1), S(1.0f));
        Quad(v, P(-1, -1, -1), P(-1, -1, 1), P(1, -1, 1), P(1, -1, -1), S(0.5f));
        Quad(v, P(-1, -1, 1), P(-1, 1, 1), P(1, 1, 1), P(1, -1, 1), S(0.8f));
        Quad(v, P(-1, -1, -1), P(1, -1, -1), P(1, 1, -1), P(-1, 1, -1), S(0.7f));
        Quad(v, P(1, -1, -1), P(1, -1, 1), P(1, 1, 1), P(1, 1, -1), S(0.9f));
        Quad(v, P(-1, -1, -1), P(-1, 1, -1), P(-1, 1, 1), P(-1, -1, 1), S(0.6f));
    }

    private static float BeamLength(Vector3 o, Vector3 d)
    {
        float t = 20f;
        foreach (var (axis, limit) in new[] { (0, RoomX), (1, RoomH), (2, RoomZ) })
        {
            float da = Axis(d, axis), oa = Axis(o, axis);
            if (da > 1e-6f) t = MathF.Min(t, (limit - oa) / da);
            else if (da < -1e-6f) t = MathF.Min(t, ((axis == 1 ? 0 : -limit) - oa) / da);
        }
        return MathF.Max(t, 0);
    }

    private static void Cone(List<float> v, Vector3 o, Vector3 d, Vector4 col)
    {
        float len = BeamLength(o, d), radius = len * MathF.Tan(BeamHalfAngle);
        var helper = MathF.Abs(d.Y) < 0.9f ? Vector3.UnitY : Vector3.UnitX;
        var a = Vector3.Normalize(Vector3.Cross(d, helper));
        var b = Vector3.Cross(d, a);
        var end = o + d * len;
        Vector3 Rim(int k)
        {
            float ang = k / (float)BeamSegments * MathF.Tau;
            return end + a * (MathF.Cos(ang) * radius) + b * (MathF.Sin(ang) * radius);
        }
        var edge = new Vector4(col.X * 0.3f, col.Y * 0.3f, col.Z * 0.3f, col.W);
        for (int k = 0; k < BeamSegments; k++)
        {
            Put(v, o, col); Put(v, Rim(k), edge); Put(v, Rim(k + 1), edge);
        }
    }

    /// Fills v with the opaque part, then the beams; returns the vertex count of the opaque part.
    public static int Build(List<float> v, Camera cam, float time, Vector3? aim)
    {
        v.Clear();
        var eye = cam.Eye;
        float x = RoomX, z = RoomZ, h = RoomH;
        Quad(v, new(-x, 0, -z), new(-x, 0, z), new(x, 0, z), new(x, 0, -z), new(0.25f, 0.25f, 0.28f, 1));
        var wall = new Vector4(0.35f, 0.33f, 0.30f, 1);
        if (WallVisible(eye, 0, -1)) Quad(v, new(-x, 0, -z), new(-x, h, -z), new(-x, h, z), new(-x, 0, z), wall);
        if (WallVisible(eye, 0, 1)) Quad(v, new(x, 0, -z), new(x, 0, z), new(x, h, z), new(x, h, -z), wall);
        var wall2 = new Vector4(0.30f, 0.30f, 0.33f, 1);
        if (WallVisible(eye, 2, -1)) Quad(v, new(-x, 0, -z), new(x, 0, -z), new(x, h, -z), new(-x, h, -z), wall2);
        if (WallVisible(eye, 2, 1)) Quad(v, new(-x, 0, z), new(-x, h, z), new(x, h, z), new(x, 0, z), wall2);
        for (int i = 0; i < Fixtures; i++)
            Cube(v, FixturePos(i), 0.1f, IsSelected(i) ? new(0.9f, 0.7f, 0.2f, 1) : new(0.6f, 0.6f, 0.6f, 1));
        if (aim is { } p0) Cube(v, p0, 0.06f, new(1, 0.1f, 0.1f, 1));
        int opaque = v.Count / 7;
        for (int i = 0; i < Fixtures; i++)
        {
            var o = FixturePos(i);
            Vector3 d;
            if (aim is { } p && IsSelected(i)) d = Vector3.Normalize(p - o);
            else
            {
                float pan = time * 0.5f + i * 0.3f, tilt = 0.5f + 0.3f * MathF.Sin(time * 0.7f + i);
                d = new(MathF.Sin(tilt) * MathF.Cos(pan), -MathF.Cos(tilt), MathF.Sin(tilt) * MathF.Sin(pan));
            }
            float hue = i / (float)Fixtures * 6.28f;
            Cone(v, o, d, new(0.5f + 0.5f * MathF.Cos(hue), 0.5f + 0.5f * MathF.Cos(hue + 2.1f), 0.5f + 0.5f * MathF.Cos(hue + 4.2f), 0.18f));
        }
        return opaque;
    }
}
